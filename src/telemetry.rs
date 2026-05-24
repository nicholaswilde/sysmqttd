use sysinfo::{System, Disks};
use std::fs;
use std::path::Path;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TelemetryState {
    pub cpu_temperature: f64,
    pub ram_usage: f64,
    pub disk_usage: f64,
}

pub struct TelemetryCollector {
    sys: System,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_memory();
        TelemetryCollector { sys }
    }

    /// Read CPU temperature from Linux sysfs with fallback options
    pub fn get_cpu_temperature(&self) -> f64 {
        // Primary source: Raspberry Pi/DietPi CPU thermal zone
        let thermal_path = "/sys/class/thermal/thermal_zone0/temp";
        if Path::new(thermal_path).exists() {
            if let Ok(content) = fs::read_to_string(thermal_path) {
                if let Ok(milli_temp) = content.trim().parse::<i32>() {
                    return (milli_temp as f64 / 1000.0 * 10.0).round() / 10.0;
                }
            }
        }

        // Secondary source: Standard hwmon devices on general Linux
        for i in 0..10 {
            let hwmon_path = format!("/sys/class/hwmon/hwmon{}/temp1_input", i);
            if Path::new(&hwmon_path).exists() {
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
}
