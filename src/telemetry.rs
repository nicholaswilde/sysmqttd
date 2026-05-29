use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use sysinfo::{Disks, System};

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TelemetryState {
    pub cpu_temperature: f64,
    pub ram_usage: f64,
    pub disk_usage: f64,
    pub cpu_usage: f64,
    pub ram_used_mb: f64,
    pub ram_free_mb: f64,
    pub disk_used_gb: f64,
    pub disk_free_gb: f64,
    #[serde(rename = "load_1m")]
    pub load_average_1: f64,
    #[serde(rename = "load_5m")]
    pub load_average_5: f64,
    #[serde(rename = "load_15m")]
    pub load_average_15: f64,
    pub uptime_seconds: f64,
    #[serde(rename = "net_rx_rate")]
    pub net_rx_rate: f64,
    #[serde(rename = "net_tx_rate")]
    pub net_tx_rate: f64,
    pub undervoltage_detected: bool,
    pub throttled: bool,
    pub ip_address: String,
    pub mac_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_rssi: Option<i32>,
    pub upgradable_packages: u32,
    pub top_process: String,
}

pub struct TelemetryCollector {
    sys: System,
    sysfs_root: PathBuf,
    prev_rx_bytes: Option<u64>,
    prev_tx_bytes: Option<u64>,
    prev_time: Option<std::time::Instant>,
    last_package_check: Option<std::time::Instant>,
    cached_package_count: u32,
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
            prev_rx_bytes: None,
            prev_tx_bytes: None,
            prev_time: None,
            last_package_check: None,
            cached_package_count: 0,
        }
    }

    /// Helper to instantiate with a custom sysfs root for testing
    pub fn with_sysfs_root(sysfs_root: PathBuf) -> Self {
        let mut sys = System::new();
        sys.refresh_memory();
        TelemetryCollector {
            sys,
            sysfs_root,
            prev_rx_bytes: None,
            prev_tx_bytes: None,
            prev_time: None,
            last_package_check: None,
            cached_package_count: 0,
        }
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

    /// Read interface cumulative RX and TX bytes from /proc/net/dev
    pub fn read_interface_bytes(&self, interface: &str) -> Result<(u64, u64), String> {
        let net_dev_path = self.sysfs_root.join("proc/net/dev");
        let content = std::fs::read_to_string(&net_dev_path).map_err(|e| e.to_string())?;
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 && parts[0].trim() == interface {
                let cols: Vec<&str> = parts[1].split_whitespace().collect();
                if cols.len() < 9 {
                    return Err(format!("Malformed columns for interface {}", interface));
                }
                let rx = cols[0].parse::<u64>().map_err(|e| e.to_string())?;
                let tx = cols[8].parse::<u64>().map_err(|e| e.to_string())?;
                return Ok((rx, tx));
            }
        }
        Err(format!(
            "Interface {} not found in /proc/net/dev",
            interface
        ))
    }
    /// Read RAM statistics: (percentage, used_mb, free_mb)
    pub fn get_ram_stats(&mut self) -> (f64, f64, f64) {
        self.sys.refresh_memory();
        let total = self.sys.total_memory();
        let used = self.sys.used_memory();
        if total == 0 {
            return (0.0, 0.0, 0.0);
        }
        let ram_pct = (used as f64 / total as f64) * 100.0;
        let used_mb = used as f64 / (1024.0 * 1024.0);
        let free = total.saturating_sub(used);
        let free_mb = free as f64 / (1024.0 * 1024.0);

        (
            (ram_pct * 10.0).round() / 10.0,
            (used_mb * 10.0).round() / 10.0,
            (free_mb * 10.0).round() / 10.0,
        )
    }

    /// Read RAM usage percentage utilizing minimized sysinfo features
    pub fn get_ram_usage(&mut self) -> f64 {
        self.get_ram_stats().0
    }

    /// Read root disk statistics: (percentage, used_gb, free_gb)
    pub fn get_disk_stats(&self) -> (f64, f64, f64) {
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            if disk.mount_point() == Path::new("/") {
                let total = disk.total_space();
                let available = disk.available_space();
                if total == 0 {
                    return (0.0, 0.0, 0.0);
                }
                let used = total.saturating_sub(available);
                let disk_pct = (used as f64 / total as f64) * 100.0;
                let used_gb = used as f64 / (1024.0 * 1024.0 * 1024.0);
                let free_gb = available as f64 / (1024.0 * 1024.0 * 1024.0);
                return (
                    (disk_pct * 10.0).round() / 10.0,
                    (used_gb * 10.0).round() / 10.0,
                    (free_gb * 10.0).round() / 10.0,
                );
            }
        }
        (0.0, 0.0, 0.0)
    }

    /// Read Disk utilization percentage for the root directory '/'
    pub fn get_disk_usage(&self) -> f64 {
        self.get_disk_stats().0
    }

    /// Read CPU utilization percentage
    pub fn get_cpu_usage(&mut self) -> f64 {
        self.sys.refresh_cpu();
        let cpu_usage = self.sys.global_cpu_info().cpu_usage();
        (cpu_usage as f64 * 10.0).round() / 10.0
    }

    /// Read SBC hardware health flags: under-voltage and thermal throttling
    pub fn get_sbc_health(&self) -> (bool, bool) {
        // Primary source: Raspberry Pi throttled bitmask
        let throttled_path = self
            .sysfs_root
            .join("sys/devices/platform/soc/soc:firmware/get_throttled");
        if throttled_path.exists() {
            if let Ok(content) = fs::read_to_string(&throttled_path) {
                let content_trimmed = content.trim();
                let parsed_val =
                    if content_trimmed.starts_with("0x") || content_trimmed.starts_with("0X") {
                        u32::from_str_radix(&content_trimmed[2..], 16).ok()
                    } else {
                        content_trimmed
                            .parse::<u32>()
                            .ok()
                            .or_else(|| u32::from_str_radix(content_trimmed, 16).ok())
                    };

                if let Some(val) = parsed_val {
                    // Bit 0: Under-voltage active
                    let undervoltage = (val & 0x1) != 0;
                    // Bit 2: Currently throttled (active thermal throttling)
                    let throttled = (val & 0x4) != 0;
                    return (undervoltage, throttled);
                }
            }
        }

        // Fallback for non-Pi Linux systems: check /sys/class/power_supply
        let power_supply_dir = self.sysfs_root.join("sys/class/power_supply");
        if power_supply_dir.exists() {
            if let Ok(entries) = fs::read_dir(&power_supply_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let health_path = path.join("health");
                    if health_path.exists() {
                        if let Ok(health) = fs::read_to_string(&health_path) {
                            let health_trimmed = health.trim().to_lowercase();
                            if health_trimmed == "overheat" {
                                return (false, true);
                            }
                        }
                    }
                }
            }
        }

        (false, false)
    }

    pub fn get_interface_ip(&self, interface: &str) -> String {
        if self.sysfs_root != Path::new("/") {
            let mock_path = self.sysfs_root.join(format!("mock_ip_{}", interface));
            if let Ok(ip) = fs::read_to_string(&mock_path) {
                return ip.trim().to_string();
            }
            return "127.0.0.1".to_string();
        }

        if let Ok(output) = std::process::Command::new("ip")
            .args(["-o", "-4", "addr", "show", "dev", interface])
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(pos) = parts.iter().position(|&x| x == "inet") {
                        if pos + 1 < parts.len() {
                            let ip_cidr = parts[pos + 1];
                            if let Some(ip) = ip_cidr.split('/').next() {
                                return ip.to_string();
                            }
                        }
                    }
                }
            }
        }

        if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") {
            if socket.connect("8.8.8.8:80").is_ok() {
                if let Ok(local_addr) = socket.local_addr() {
                    return local_addr.ip().to_string();
                }
            }
        }

        "0.0.0.0".to_string()
    }

    pub fn get_interface_mac(&self, interface: &str) -> String {
        let mac_path = self
            .sysfs_root
            .join(format!("sys/class/net/{}/address", interface));
        if mac_path.exists() {
            if let Ok(content) = fs::read_to_string(&mac_path) {
                return content.trim().to_string();
            }
        }
        "00:00:00:00:00:00".to_string()
    }

    pub fn get_wifi_rssi(&self, interface: &str) -> Option<i32> {
        let wireless_path = self.sysfs_root.join("proc/net/wireless");
        if wireless_path.exists() {
            if let Ok(content) = fs::read_to_string(&wireless_path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with(interface) {
                        let parts: Vec<&str> = trimmed.split_whitespace().collect();
                        if parts.len() > 3 {
                            let rssi_str = parts[3].trim_end_matches('.');
                            if let Ok(rssi) = rssi_str.parse::<i32>() {
                                return Some(rssi);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_upgradable_packages(&self) -> u32 {
        if self.sysfs_root != Path::new("/") {
            return 3;
        }

        if let Ok(output) = std::process::Command::new("apt-get")
            .args(["-s", "upgrade"])
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let count = stdout
                    .lines()
                    .filter(|line| line.starts_with("Inst "))
                    .count();
                return count as u32;
            }
        }
        0
    }

    pub fn get_top_process(&mut self) -> String {
        self.sys.refresh_processes();
        let mut top_proc: Option<(&sysinfo::Process, f32)> = None;

        for proc in self.sys.processes().values() {
            let cpu = proc.cpu_usage();
            if let Some((_, top_cpu)) = top_proc {
                if cpu > top_cpu {
                    top_proc = Some((proc, cpu));
                }
            } else {
                top_proc = Some((proc, cpu));
            }
        }

        if let Some((proc, cpu)) = top_proc {
            if cpu > 0.1 {
                let mem_mb = proc.memory() as f64 / (1024.0 * 1024.0);
                return format!(
                    "{} ({}) - {:.1}% CPU, {:.1} MB RAM",
                    proc.name(),
                    proc.pid(),
                    cpu,
                    mem_mb
                );
            }
        }

        // Fallback to top memory consumer if CPU is very low/zero
        let mut top_mem_proc: Option<&sysinfo::Process> = None;
        for proc in self.sys.processes().values() {
            if let Some(top_mem) = top_mem_proc {
                if proc.memory() > top_mem.memory() {
                    top_mem_proc = Some(proc);
                }
            } else {
                top_mem_proc = Some(proc);
            }
        }

        if let Some(proc) = top_mem_proc {
            let mem_mb = proc.memory() as f64 / (1024.0 * 1024.0);
            return format!(
                "{} ({}) - {:.1}% CPU, {:.1} MB RAM",
                proc.name(),
                proc.pid(),
                proc.cpu_usage(),
                mem_mb
            );
        }

        "None".to_string()
    }

    /// Collect all metrics
    pub fn collect(&mut self, interface: &str) -> TelemetryState {
        let (load1, load5, load15) = self.read_load_avg().unwrap_or((0.0, 0.0, 0.0));

        let mut net_rx_rate = 0.0;
        let mut net_tx_rate = 0.0;

        if let Ok((curr_rx, curr_tx)) = self.read_interface_bytes(interface) {
            let now = std::time::Instant::now();
            if let (Some(prev_rx), Some(prev_tx), Some(prev_t)) =
                (self.prev_rx_bytes, self.prev_tx_bytes, self.prev_time)
            {
                let delta_secs = now.duration_since(prev_t).as_secs_f64();
                if delta_secs > 0.0 {
                    let rx_delta = curr_rx.saturating_sub(prev_rx) as f64;
                    let tx_delta = curr_tx.saturating_sub(prev_tx) as f64;
                    let rx_rate = (rx_delta / delta_secs) / 1024.0;
                    let tx_rate = (tx_delta / delta_secs) / 1024.0;
                    net_rx_rate = (rx_rate * 10.0).round() / 10.0;
                    net_tx_rate = (tx_rate * 10.0).round() / 10.0;
                }
            }
            self.prev_rx_bytes = Some(curr_rx);
            self.prev_tx_bytes = Some(curr_tx);
            self.prev_time = Some(now);
        }

        let (ram_usage, ram_used_mb, ram_free_mb) = self.get_ram_stats();
        let (disk_usage, disk_used_gb, disk_free_gb) = self.get_disk_stats();
        let (undervoltage_detected, throttled) = self.get_sbc_health();
        let ip_address = self.get_interface_ip(interface);
        let mac_address = self.get_interface_mac(interface);
        let wifi_rssi = self.get_wifi_rssi(interface);

        // Daily/slow-loop check for package updates
        let now = std::time::Instant::now();
        let should_check = match self.last_package_check {
            None => true,
            Some(last) => now.duration_since(last) >= std::time::Duration::from_secs(86400),
        };

        if should_check {
            self.cached_package_count = self.get_upgradable_packages();
            self.last_package_check = Some(now);
        }
        let upgradable_packages = self.cached_package_count;
        let top_process = self.get_top_process();

        TelemetryState {
            cpu_temperature: self.get_cpu_temperature(),
            ram_usage,
            disk_usage,
            cpu_usage: self.get_cpu_usage(),
            ram_used_mb,
            ram_free_mb,
            disk_used_gb,
            disk_free_gb,
            load_average_1: (load1 * 10.0).round() / 10.0,
            load_average_5: (load5 * 10.0).round() / 10.0,
            load_average_15: (load15 * 10.0).round() / 10.0,
            uptime_seconds: (self.read_uptime().unwrap_or(0.0) * 10.0).round() / 10.0,
            net_rx_rate,
            net_tx_rate,
            undervoltage_detected,
            throttled,
            ip_address,
            mac_address,
            wifi_rssi,
            upgradable_packages,
            top_process,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_collection() {
        let mut collector = TelemetryCollector::new();
        let state = collector.collect("wlan0");

        // Assertions verifying that outputs are plausible percentages/temperatures
        assert!(state.cpu_temperature >= -40.0 && state.cpu_temperature <= 120.0);
        assert!(state.ram_usage >= 0.0 && state.ram_usage <= 100.0);
        assert!(state.disk_usage >= 0.0 && state.disk_usage <= 100.0);
        assert!(state.cpu_usage >= 0.0 && state.cpu_usage <= 100.0);
    }

    #[test]
    fn test_telemetry_collection_with_bandwidth_rates() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_telemetry_rates");
        let proc_dir = test_dir.join("proc/net");
        std::fs::create_dir_all(&proc_dir).unwrap();
        let net_dev_file = proc_dir.parent().unwrap().join("net/dev");

        let mock_content_1 =
            "Inter-|   Receive\n face |bytes\n wlan0: 1000 10 0 0 0 0 0 0 2000 20 0 0 0 0 0 0\n";
        std::fs::write(&net_dev_file, mock_content_1).unwrap();

        let mut collector = TelemetryCollector::with_sysfs_root(test_dir.clone());

        // First collection establishes base/prev values
        let state_1 = collector.collect("wlan0");
        assert_eq!(state_1.net_rx_rate, 0.0);
        assert_eq!(state_1.net_tx_rate, 0.0);

        // Advance time manually by modifying collector's prev_time
        let past_time = std::time::Instant::now() - std::time::Duration::from_secs(2);
        collector.prev_time = Some(past_time);

        // Update mock content with more bytes
        // RX delta = 2048 bytes (2 kB) -> Rate over 2s = 1 kB/s
        // TX delta = 4096 bytes (4 kB) -> Rate over 2s = 2 kB/s
        let mock_content_2 =
            "Inter-|   Receive\n face |bytes\n wlan0: 3048 10 0 0 0 0 0 0 6096 20 0 0 0 0 0 0\n";
        std::fs::write(&net_dev_file, mock_content_2).unwrap();

        let state_2 = collector.collect("wlan0");
        assert!((state_2.net_rx_rate - 1.0).abs() < 0.1);
        assert!((state_2.net_tx_rate - 2.0).abs() < 0.1);

        // Clean up
        let _ = std::fs::remove_file(net_dev_file);
        let _ = std::fs::remove_dir_all(test_dir);
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

    #[test]
    fn test_read_interface_bytes() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_net_bandwidth");
        let proc_dir = test_dir.join("proc/net");
        std::fs::create_dir_all(&proc_dir).unwrap();
        let net_dev_file = proc_dir.parent().unwrap().join("net/dev");

        let mock_content = "Inter-|   Receive                                                |  Transmit\n\
                             face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed\n\
                              lo: 1000 10 0 0 0 0 0 0 2000 20 0 0 0 0 0 0\n\
                              wlan0: 150000 150 0 1 0 0 0 0 250000 250 0 0 0 0 0 0\n";
        std::fs::write(&net_dev_file, mock_content).unwrap();

        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let (rx, tx) = collector.read_interface_bytes("wlan0").unwrap();
        assert_eq!(rx, 150000);
        assert_eq!(tx, 250000);

        let (lo_rx, lo_tx) = collector.read_interface_bytes("lo").unwrap();
        assert_eq!(lo_rx, 1000);
        assert_eq!(lo_tx, 2000);

        // Test non-existent interface
        assert!(collector.read_interface_bytes("eth0").is_err());

        // Test malformed columns
        let malformed_content = "Inter-|   Receive\n face |bytes\n wlan0: 1500\n";
        std::fs::write(&net_dev_file, malformed_content).unwrap();
        assert!(collector.read_interface_bytes("wlan0").is_err());

        // Test invalid parsing values
        let invalid_val_content =
            "Inter-|   Receive\n face |bytes\n wlan0: abc 10 0 0 0 0 0 0 2000 20 0 0 0 0 0 0\n";
        std::fs::write(&net_dev_file, invalid_val_content).unwrap();
        assert!(collector.read_interface_bytes("wlan0").is_err());

        // Clean up
        let _ = std::fs::remove_file(net_dev_file);
        let _ = std::fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_ram_and_disk_stats() {
        let mut collector = TelemetryCollector::new();

        let (ram_pct, ram_used_mb, ram_free_mb) = collector.get_ram_stats();
        assert!((0.0..=100.0).contains(&ram_pct));
        assert!(ram_used_mb >= 0.0);
        assert!(ram_free_mb >= 0.0);

        let (disk_pct, disk_used_gb, disk_free_gb) = collector.get_disk_stats();
        assert!((0.0..=100.0).contains(&disk_pct));
        assert!(disk_used_gb >= 0.0);
        assert!(disk_free_gb >= 0.0);

        let state = collector.collect("wlan0");
        assert!((0.0..=100.0).contains(&state.ram_usage));
        assert!(state.ram_used_mb >= 0.0);
        assert!(state.ram_free_mb >= 0.0);
        assert!((0.0..=100.0).contains(&state.disk_usage));
        assert!(state.disk_used_gb >= 0.0);
        assert!(state.disk_free_gb >= 0.0);
    }

    #[test]
    fn test_sbc_health_pi_hex() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_sbc_hex");
        let fw_dir = test_dir.join("sys/devices/platform/soc/soc:firmware");
        fs::create_dir_all(&fw_dir).unwrap();

        let throttled_file = fw_dir.join("get_throttled");
        // Active under-voltage (bit 0) and active throttled (bit 2) -> 0x5
        fs::write(&throttled_file, "0x5\n").unwrap();

        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let (uv, thr) = collector.get_sbc_health();
        assert!(uv);
        assert!(thr);

        // Neither active -> 0x50000 (past undervoltage/throttling but not active)
        fs::write(&throttled_file, "0x50000\n").unwrap();
        let (uv2, thr2) = collector.get_sbc_health();
        assert!(!uv2);
        assert!(!thr2);

        // Clean up
        let _ = fs::remove_file(throttled_file);
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_sbc_health_pi_decimal() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_sbc_dec");
        let fw_dir = test_dir.join("sys/devices/platform/soc/soc:firmware");
        fs::create_dir_all(&fw_dir).unwrap();

        let throttled_file = fw_dir.join("get_throttled");
        // Active under-voltage (bit 0) -> 1
        fs::write(&throttled_file, "1\n").unwrap();

        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let (uv, thr) = collector.get_sbc_health();
        assert!(uv);
        assert!(!thr);

        // Active throttled (bit 2) -> 4
        fs::write(&throttled_file, "4\n").unwrap();
        let (uv2, thr2) = collector.get_sbc_health();
        assert!(!uv2);
        assert!(thr2);

        // Clean up
        let _ = fs::remove_file(throttled_file);
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_sbc_health_non_pi_fallback() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_sbc_non_pi");
        let power_supply_ac = test_dir.join("sys/class/power_supply/AC");
        fs::create_dir_all(&power_supply_ac).unwrap();

        let health_file = power_supply_ac.join("health");
        fs::write(&health_file, "Overheat\n").unwrap();

        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let (uv, thr) = collector.get_sbc_health();
        assert!(!uv);
        assert!(thr);

        // Standard healthy battery/AC -> health is "Good" or "Normal"
        fs::write(&health_file, "Good\n").unwrap();
        let (uv2, thr2) = collector.get_sbc_health();
        assert!(!uv2);
        assert!(!thr2);

        // Clean up
        let _ = fs::remove_file(health_file);
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_sbc_health_fallback_clean() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_sbc_clean");
        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let (uv, thr) = collector.get_sbc_health();
        assert!(!uv);
        assert!(!thr);
    }

    #[test]
    fn test_network_diagnostics_mock() {
        let test_dir = std::env::temp_dir().join("sysmqttd_test_net_diag");
        let sys_net_wlan = test_dir.join("sys/class/net/wlan0");
        fs::create_dir_all(&sys_net_wlan).unwrap();

        let address_file = sys_net_wlan.join("address");
        fs::write(&address_file, "aa:bb:cc:dd:ee:ff\n").unwrap();

        let mock_ip_file = test_dir.join("mock_ip_wlan0");
        fs::write(&mock_ip_file, "192.168.1.150\n").unwrap();

        let proc_dir = test_dir.join("proc");
        fs::create_dir_all(&proc_dir).unwrap();
        let wireless_file = proc_dir.join("net/wireless");
        fs::create_dir_all(wireless_file.parent().unwrap()).unwrap();
        fs::write(&wireless_file, "Inter-| sta-| Quality        | Discarded packets               | Missed | WE\n face |tus | link level noise | nwid crypt frag retry misc | beacon | %d\n  wlan0: 0000   45.  -65.  -256.        0      0     0      0      0      0        0\n").unwrap();

        let collector = TelemetryCollector::with_sysfs_root(test_dir.clone());
        let ip = collector.get_interface_ip("wlan0");
        let mac = collector.get_interface_mac("wlan0");
        let rssi = collector.get_wifi_rssi("wlan0");

        assert_eq!(ip, "192.168.1.150");
        assert_eq!(mac, "aa:bb:cc:dd:ee:ff");
        assert_eq!(rssi, Some(-65));

        // Clean up
        let _ = fs::remove_dir_all(test_dir);
    }
}
