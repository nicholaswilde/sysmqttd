use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use sysinfo::{Disks, System};

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TelemetryState {
    pub cpu_temperature: f64,
    pub ram_usage: f64,
    pub disk_usage: f64,
    #[serde(rename = "load_1m")]
    pub load_average_1: f64,
    #[serde(rename = "load_5m")]
    pub load_average_5: f64,
    #[serde(rename = "load_15m")]
    pub load_average_15: f64,
    pub uptime_seconds: f64,
}

pub struct TelemetryCollector {
    sys: System,
    sysfs_root: PathBuf,
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetryCollector {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_memory();
        TelemetryCollector {
            sys,
            sysfs_root: PathBuf::from("/"),
        }
    }

    /// Helper to instantiate with a custom sysfs root for testing
    pub fn with_sysfs_root(sysfs_root: PathBuf) -> Self {
        let mut sys = System::new();
        sys.refresh_memory();
        TelemetryCollector { sys, sysfs_root }
    }

    /// Read CPU temperature from Linux sysfs with fallback options
    pub fn get_cpu_temperature(&self) -> f64 {
        // Primary source: Raspberry Pi/DietPi CPU thermal zone
        let thermal_path = self.sysfs_root.join("sys/class/thermal/thermal_zone0/temp");
        if thermal_path.exists() {
            if let Ok(content) = fs::read_to_string(&thermal_path) {
                if let Ok(milli_temp) = content.trim().parse::<i32>() {
                    return (milli_temp as f64 / 1000.0 * 10.0).round() / 10.0;
                }
            }
        }

        // Secondary source: Standard hwmon devices on general Linux
        for i in 0..10 {
            let hwmon_path = self
                .sysfs_root
                .join(format!("sys/class/hwmon/hwmon{}/temp1_input", i));
            if hwmon_path.exists() {
                if let Ok(content) = fs::read_to_string(&hwmon_path) {
                    if let Ok(milli_temp) = content.trim().parse::<i32>() {
                        return (milli_temp as f64 / 1000.0 * 10.0).round() / 10.0;
                    }
                }
            }
        }

        // Fallback mock value for local non-Linux or VM environments
        42.0
    }

    /// Read system uptime in seconds from /proc/uptime
    pub fn read_uptime(&self) -> Result<f64, String> {
        let uptime_path = self.sysfs_root.join("proc/uptime");
        let content = std::fs::read_to_string(&uptime_path).map_err(|e| e.to_string())?;
        let first = content
            .split_whitespace()
            .next()
            .ok_or("Empty /proc/uptime")?;
        let seconds: f64 = first.parse::<f64>().map_err(|e| e.to_string())?;
        Ok(seconds)
    }

    /// Read load averages from /proc/loadavg
    pub fn read_load_avg(&self) -> Result<(f64, f64, f64), String> {
        let load_path = self.sysfs_root.join("proc/loadavg");
        let content = std::fs::read_to_string(&load_path).map_err(|e| e.to_string())?;
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() < 3 {
            return Err("Malformed /proc/loadavg".into());
        }
        let one = parts[0].parse::<f64>().map_err(|e| e.to_string())?;
        let five = parts[1].parse::<f64>().map_err(|e| e.to_string())?;
        let fifteen = parts[2].parse::<f64>().map_err(|e| e.to_string())?;
        Ok((one, five, fifteen))
    }
    /// Read RAM usage percentage utilizing minimized sysinfo features
    pub fn get_ram_usage(&mut self) -> f64 {
        self.sys.refresh_memory();
        let total = self.sys.total_memory();
        let used = self.sys.used_memory();
        if total == 0 {
            return 0.0;
        }
        let ram_pct = (used as f64 / total as f64) * 100.0;
        (ram_pct * 10.0).round() / 10.0
    }

    /// Read Disk utilization percentage for the root directory '/'
    pub fn get_disk_usage(&self) -> f64 {
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            if disk.mount_point() == Path::new("/") {
                let total = disk.total_space();
                let available = disk.available_space();
                if total == 0 {
                    return 0.0;
                }
                let used = total.saturating_sub(available);
                let disk_pct = (used as f64 / total as f64) * 100.0;
                return (disk_pct * 10.0).round() / 10.0;
            }
        }
        0.0
    }

    /// Collect all metrics
    pub fn collect(&mut self) -> TelemetryState {
        let (load1, load5, load15) = self.read_load_avg().unwrap_or((0.0, 0.0, 0.0));
        TelemetryState {
            cpu_temperature: self.get_cpu_temperature(),
            ram_usage: self.get_ram_usage(),
            disk_usage: self.get_disk_usage(),
            load_average_1: load1,
            load_average_5: load5,
            load_average_15: load15,
            uptime_seconds: self.read_uptime().unwrap_or(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_collection() {
        let mut collector = TelemetryCollector::new();
        let state = collector.collect();

        // Assertions verifying that outputs are plausible percentages/temperatures
        assert!(state.cpu_temperature >= -40.0 && state.cpu_temperature <= 120.0);
        assert!(state.ram_usage >= 0.0 && state.ram_usage <= 100.0);
        assert!(state.disk_usage >= 0.0 && state.disk_usage <= 100.0);
    }

    #[test]
    fn test_cpu_temp_thermal_zone() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_thermal");
        let thermal_dir = test_dir.join("sys/class/thermal/thermal_zone0");
        fs::create_dir_all(&thermal_dir).unwrap();

        let temp_file = thermal_dir.join("temp");
        fs::write(&temp_file, "45600\n").unwrap();

        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let temp = collector.get_cpu_temperature();
        assert_eq!(temp, 45.6);

        // Clean up
        let _ = fs::remove_file(temp_file);
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_cpu_temp_hwmon() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_hwmon");
        let hwmon_dir = test_dir.join("sys/class/hwmon/hwmon3");
        fs::create_dir_all(&hwmon_dir).unwrap();

        let temp_file = hwmon_dir.join("temp1_input");
        fs::write(&temp_file, "37200\n").unwrap();

        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let temp = collector.get_cpu_temperature();
        assert_eq!(temp, 37.2);

        // Clean up
        let _ = fs::remove_file(temp_file);
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_cpu_temp_fallback() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_fallback");
        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let temp = collector.get_cpu_temperature();
        assert_eq!(temp, 42.0);
    }

    #[test]
    fn test_cpu_temp_invalid_hwmon() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_invalid_hwmon");
        let hwmon_dir = test_dir.join("sys/class/hwmon/hwmon3");
        fs::create_dir_all(&hwmon_dir).unwrap();

        let temp_file = hwmon_dir.join("temp1_input");
        fs::write(&temp_file, "abc\n").unwrap();

        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let temp = collector.get_cpu_temperature();
        assert_eq!(temp, 42.0); // should fallback to 42.0

        // Clean up
        let _ = fs::remove_file(temp_file);
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_read_uptime() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_uptime");
        let proc_dir = test_dir.join("proc");
        std::fs::create_dir_all(&proc_dir).unwrap();
        let uptime_file = proc_dir.join("uptime");
        std::fs::write(&uptime_file, "12345.67 0.00\n").unwrap();

        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let uptime = collector.read_uptime().unwrap();
        assert!((uptime - 12345.67).abs() < 0.001);

        // Test empty uptime
        std::fs::write(&uptime_file, "\n").unwrap();
        assert!(collector.read_uptime().is_err());

        // Test invalid float uptime
        std::fs::write(&uptime_file, "abc 0.00\n").unwrap();
        assert!(collector.read_uptime().is_err());

        // Clean up
        let _ = std::fs::remove_file(uptime_file);
        let _ = std::fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_read_load_avg() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_load_avg");
        let proc_dir = test_dir.join("proc");
        std::fs::create_dir_all(&proc_dir).unwrap();
        let load_file = proc_dir.join("loadavg");
        std::fs::write(&load_file, "0.15 0.25 0.35 1/140 12345\n").unwrap();

        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let (one, five, fifteen) = collector.read_load_avg().unwrap();
        assert!((one - 0.15).abs() < 0.001);
        assert!((five - 0.25).abs() < 0.001);
        assert!((fifteen - 0.35).abs() < 0.001);

        // Test malformed loadavg file
        std::fs::write(&load_file, "0.15 0.25\n").unwrap();
        assert!(collector.read_load_avg().is_err());

        // Test invalid float values
        std::fs::write(&load_file, "0.15 abc 0.35 1/140 12345\n").unwrap();
        assert!(collector.read_load_avg().is_err());

        // Clean up
        let _ = std::fs::remove_file(load_file);
        let _ = std::fs::remove_dir_all(test_dir);
    }
}
