use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use sysinfo::{Disks, System};

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TelemetryState {
    pub cpu_temperature: f64,
    pub ram_usage: f64,
    pub disk_usage: f64,
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
        TelemetryState {
            cpu_temperature: self.get_cpu_temperature(),
            ram_usage: self.get_ram_usage(),
            disk_usage: self.get_disk_usage(),
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
}
