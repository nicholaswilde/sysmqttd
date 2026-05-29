use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct DeviceInfo {
    #[serde(rename = "ids")]
    pub identifiers: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "mdl")]
    pub model: String,
    #[serde(rename = "mf")]
    pub manufacturer: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct DiscoveryPayload {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "stat_t")]
    pub state_topic: String,
    #[serde(rename = "avty_t")]
    pub availability_topic: String,
    #[serde(rename = "val_tpl")]
    pub value_template: String,
    #[serde(rename = "unit_of_meas", skip_serializing_if = "Option::is_none")]
    pub unit_of_measurement: Option<String>,
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,
    #[serde(rename = "state_class", skip_serializing_if = "Option::is_none")]
    pub state_class: Option<String>,
    #[serde(rename = "uniq_id")]
    pub unique_id: String,
    #[serde(rename = "dev")]
    pub device: DeviceInfo,
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<String>,
}

impl DiscoveryPayload {
    pub fn new_cpu_temp(prefix: &str, hostname: &str, unit: &str, device: DeviceInfo) -> Self {
        let unit_str = if unit == "F" { "°F" } else { "°C" };
        DiscoveryPayload {
            name: "CPU Temperature".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.cpu_temperature }}".to_string(),
            unit_of_measurement: Some(unit_str.to_string()),
            device_class: Some("temperature".to_string()),
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_cpu_temp", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_cpu_usage(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "CPU Usage".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.cpu_usage }}".to_string(),
            unit_of_measurement: Some("%".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_cpu_usage", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_ram_usage(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "RAM Usage".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.ram_usage }}".to_string(),
            unit_of_measurement: Some("%".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_ram_usage", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_ram_used(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "RAM Used".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.ram_used_mb }}".to_string(),
            unit_of_measurement: Some("MB".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_ram_used", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_ram_free(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "RAM Free".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.ram_free_mb }}".to_string(),
            unit_of_measurement: Some("MB".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_ram_free", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_disk_usage(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Disk Storage Utilization".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.disk_usage }}".to_string(),
            unit_of_measurement: Some("%".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_disk_usage", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_disk_used(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Disk Used".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.disk_used_gb }}".to_string(),
            unit_of_measurement: Some("GB".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_disk_used", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_disk_free(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Disk Free".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.disk_free_gb }}".to_string(),
            unit_of_measurement: Some("GB".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_disk_free", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_load_1m(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Load Avg (1m)".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.load_1m }}".to_string(),
            unit_of_measurement: Some("".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_load_1m", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_load_5m(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Load Avg (5m)".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.load_5m }}".to_string(),
            unit_of_measurement: Some("".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_load_5m", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_load_15m(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Load Avg (15m)".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.load_15m }}".to_string(),
            unit_of_measurement: Some("".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_load_15m", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_net_rx_rate(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Network RX Rate".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.net_rx_rate }}".to_string(),
            unit_of_measurement: Some("kB/s".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_net_rx_rate", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_net_tx_rate(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Network TX Rate".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.net_tx_rate }}".to_string(),
            unit_of_measurement: Some("kB/s".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_net_tx_rate", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_uptime(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Uptime".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.uptime_seconds }}".to_string(),
            unit_of_measurement: Some("s".to_string()),
            device_class: Some("duration".to_string()),
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_uptime", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_undervoltage(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Under-voltage Detected".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ 'ON' if value_json.undervoltage_detected else 'OFF' }}".to_string(),
            unit_of_measurement: None,
            device_class: Some("problem".to_string()),
            state_class: None,
            unique_id: format!("sysmqttd_{}_undervoltage", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_throttled(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Throttled".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ 'ON' if value_json.throttled else 'OFF' }}".to_string(),
            unit_of_measurement: None,
            device_class: Some("problem".to_string()),
            state_class: None,
            unique_id: format!("sysmqttd_{}_throttled", hostname),
            device,
            entity_category: None,
        }
    }

    pub fn new_ip_address(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "IP Address".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.ip_address }}".to_string(),
            unit_of_measurement: None,
            device_class: None,
            state_class: None,
            unique_id: format!("sysmqttd_{}_ip_address", hostname),
            device,
            entity_category: Some("diagnostic".to_string()),
        }
    }

    pub fn new_mac_address(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "MAC Address".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.mac_address }}".to_string(),
            unit_of_measurement: None,
            device_class: None,
            state_class: None,
            unique_id: format!("sysmqttd_{}_mac_address", hostname),
            device,
            entity_category: Some("diagnostic".to_string()),
        }
    }

    pub fn new_wifi_rssi(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Wi-Fi RSSI".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.wifi_rssi }}".to_string(),
            unit_of_measurement: Some("dBm".to_string()),
            device_class: Some("signal_strength".to_string()),
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_wifi_rssi", hostname),
            device,
            entity_category: Some("diagnostic".to_string()),
        }
    }

    pub fn new_upgradable_packages(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Upgradable Packages".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.upgradable_packages }}".to_string(),
            unit_of_measurement: Some("packages".to_string()),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unique_id: format!("sysmqttd_{}_upgradable_packages", hostname),
            device,
            entity_category: Some("diagnostic".to_string()),
        }
    }

    pub fn new_top_process(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Top Process".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.top_process }}".to_string(),
            unit_of_measurement: None,
            device_class: None,
            state_class: None,
            unique_id: format!("sysmqttd_{}_top_process", hostname),
            device,
            entity_category: Some("diagnostic".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_upgradable_packages_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "System Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };
        let payload =
            DiscoveryPayload::new_upgradable_packages("homeassistant", "test-host", device);
        let serialized = serde_json::to_string(&payload).unwrap();

        assert!(serialized.contains(r#""name":"Upgradable Packages""#));
        assert!(serialized.contains(r#""stat_t":"homeassistant/sensor/sysmqttd_test-host/state""#));
        assert!(serialized.contains(r#""val_tpl":"{{ value_json.upgradable_packages }}""#));
        assert!(serialized.contains(r#""unit_of_meas":"packages""#));
        assert!(serialized.contains(r#""state_class":"measurement""#));
        assert!(serialized.contains(r#""ent_cat":"diagnostic""#));
    }

    #[test]
    fn test_top_process_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "System Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };
        let payload = DiscoveryPayload::new_top_process("homeassistant", "test-host", device);
        let serialized = serde_json::to_string(&payload).unwrap();

        assert!(serialized.contains(r#""name":"Top Process""#));
        assert!(serialized.contains(r#""stat_t":"homeassistant/sensor/sysmqttd_test-host/state""#));
        assert!(serialized.contains(r#""val_tpl":"{{ value_json.top_process }}""#));
        assert!(serialized.contains(r#""ent_cat":"diagnostic""#));
    }

    #[test]
    fn test_cpu_temp_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "System Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };
        let payload_c =
            DiscoveryPayload::new_cpu_temp("homeassistant", "test-host", "C", device.clone());
        let serialized_c = serde_json::to_string(&payload_c).unwrap();

        // Assertions verifying exact keys and structure
        assert!(serialized_c.contains(r#""name":"CPU Temperature""#));
        assert!(
            serialized_c.contains(r#""stat_t":"homeassistant/sensor/sysmqttd_test-host/state""#)
        );
        assert!(serialized_c
            .contains(r#""avty_t":"homeassistant/sensor/sysmqttd_test-host/availability""#));
        assert!(serialized_c.contains(r#""val_tpl":"{{ value_json.cpu_temperature }}""#));
        assert!(serialized_c.contains(r#""unit_of_meas":"°C""#));
        assert!(serialized_c.contains(r#""dev_cla":"temperature""#));
        assert!(serialized_c.contains(r#""state_class":"measurement""#));
        assert!(serialized_c.contains(r#""uniq_id":"sysmqttd_test-host_cpu_temp""#));
        assert!(serialized_c.contains(r#""dev":{"ids":["sysmqttd_test-host"],"name":"sysmqttd test-host","mdl":"System Monitor","mf":"sysmqttd"}"#));

        // Fahrenheit test
        let payload_f = DiscoveryPayload::new_cpu_temp("homeassistant", "test-host", "F", device);
        let serialized_f = serde_json::to_string(&payload_f).unwrap();
        assert!(serialized_f.contains(r#""unit_of_meas":"°F""#));
    }

    #[test]
    fn test_load_avg_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "System Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };
        let payload_1m =
            DiscoveryPayload::new_load_1m("homeassistant", "test-host", device.clone());
        let payload_5m =
            DiscoveryPayload::new_load_5m("homeassistant", "test-host", device.clone());
        let payload_15m = DiscoveryPayload::new_load_15m("homeassistant", "test-host", device);

        let s1 = serde_json::to_string(&payload_1m).unwrap();
        let s5 = serde_json::to_string(&payload_5m).unwrap();
        let s15 = serde_json::to_string(&payload_15m).unwrap();

        assert!(s1.contains(r#""name":"Load Avg (1m)""#));
        assert!(s1.contains(r#""val_tpl":"{{ value_json.load_1m }}""#));
        assert!(s1.contains(r#""uniq_id":"sysmqttd_test-host_load_1m""#));

        assert!(s5.contains(r#""name":"Load Avg (5m)""#));
        assert!(s5.contains(r#""val_tpl":"{{ value_json.load_5m }}""#));
        assert!(s5.contains(r#""uniq_id":"sysmqttd_test-host_load_5m""#));

        assert!(s15.contains(r#""name":"Load Avg (15m)""#));
        assert!(s15.contains(r#""val_tpl":"{{ value_json.load_15m }}""#));
        assert!(s15.contains(r#""uniq_id":"sysmqttd_test-host_load_15m""#));
    }

    #[test]
    fn test_net_rate_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "System Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };
        let payload_rx =
            DiscoveryPayload::new_net_rx_rate("homeassistant", "test-host", device.clone());
        let payload_tx = DiscoveryPayload::new_net_tx_rate("homeassistant", "test-host", device);

        let s_rx = serde_json::to_string(&payload_rx).unwrap();
        let s_tx = serde_json::to_string(&payload_tx).unwrap();

        assert!(s_rx.contains(r#""name":"Network RX Rate""#));
        assert!(s_rx.contains(r#""val_tpl":"{{ value_json.net_rx_rate }}""#));
        assert!(s_rx.contains(r#""unit_of_meas":"kB/s""#));
        assert!(s_rx.contains(r#""uniq_id":"sysmqttd_test-host_net_rx_rate""#));

        assert!(s_tx.contains(r#""name":"Network TX Rate""#));
        assert!(s_tx.contains(r#""val_tpl":"{{ value_json.net_tx_rate }}""#));
        assert!(s_tx.contains(r#""unit_of_meas":"kB/s""#));
        assert!(s_tx.contains(r#""uniq_id":"sysmqttd_test-host_net_tx_rate""#));
    }

    #[test]
    fn test_uptime_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "System Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };
        let payload = DiscoveryPayload::new_uptime("homeassistant", "test-host", device);
        let s = serde_json::to_string(&payload).unwrap();

        assert!(s.contains(r#""name":"Uptime""#));
        assert!(s.contains(r#""val_tpl":"{{ value_json.uptime_seconds }}""#));
        assert!(s.contains(r#""unit_of_meas":"s""#));
        assert!(s.contains(r#""dev_cla":"duration""#));
        assert!(s.contains(r#""uniq_id":"sysmqttd_test-host_uptime""#));
    }

    #[test]
    fn test_cpu_usage_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "System Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };
        let payload = DiscoveryPayload::new_cpu_usage("homeassistant", "test-host", device);
        let s = serde_json::to_string(&payload).unwrap();

        assert!(s.contains(r#""name":"CPU Usage""#));
        assert!(s.contains(r#""val_tpl":"{{ value_json.cpu_usage }}""#));
        assert!(s.contains(r#""unit_of_meas":"%""#));
        assert!(s.contains(r#""uniq_id":"sysmqttd_test-host_cpu_usage""#));
    }

    #[test]
    fn test_absolute_telemetry_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "System Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };

        let ram_used = DiscoveryPayload::new_ram_used("homeassistant", "test-host", device.clone());
        let ram_free = DiscoveryPayload::new_ram_free("homeassistant", "test-host", device.clone());
        let disk_used =
            DiscoveryPayload::new_disk_used("homeassistant", "test-host", device.clone());
        let disk_free = DiscoveryPayload::new_disk_free("homeassistant", "test-host", device);

        let s_ru = serde_json::to_string(&ram_used).unwrap();
        let s_rf = serde_json::to_string(&ram_free).unwrap();
        let s_du = serde_json::to_string(&disk_used).unwrap();
        let s_df = serde_json::to_string(&disk_free).unwrap();

        assert!(s_ru.contains(r#""name":"RAM Used""#));
        assert!(s_ru.contains(r#""val_tpl":"{{ value_json.ram_used_mb }}""#));
        assert!(s_ru.contains(r#""unit_of_meas":"MB""#));
        assert!(s_ru.contains(r#""uniq_id":"sysmqttd_test-host_ram_used""#));

        assert!(s_rf.contains(r#""name":"RAM Free""#));
        assert!(s_rf.contains(r#""val_tpl":"{{ value_json.ram_free_mb }}""#));
        assert!(s_rf.contains(r#""unit_of_meas":"MB""#));
        assert!(s_rf.contains(r#""uniq_id":"sysmqttd_test-host_ram_free""#));

        assert!(s_du.contains(r#""name":"Disk Used""#));
        assert!(s_du.contains(r#""val_tpl":"{{ value_json.disk_used_gb }}""#));
        assert!(s_du.contains(r#""unit_of_meas":"GB""#));
        assert!(s_du.contains(r#""uniq_id":"sysmqttd_test-host_disk_used""#));

        assert!(s_df.contains(r#""name":"Disk Free""#));
        assert!(s_df.contains(r#""val_tpl":"{{ value_json.disk_free_gb }}""#));
        assert!(s_df.contains(r#""unit_of_meas":"GB""#));
        assert!(s_df.contains(r#""uniq_id":"sysmqttd_test-host_disk_free""#));
    }

    #[test]
    fn test_binary_sensor_discovery_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "System Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };

        let uv_payload =
            DiscoveryPayload::new_undervoltage("homeassistant", "test-host", device.clone());
        let thr_payload = DiscoveryPayload::new_throttled("homeassistant", "test-host", device);

        let s_uv = serde_json::to_string(&uv_payload).unwrap();
        let s_thr = serde_json::to_string(&thr_payload).unwrap();

        // Under-voltage Detected
        assert!(s_uv.contains(r#""name":"Under-voltage Detected""#));
        assert!(s_uv
            .contains(r#""val_tpl":"{{ 'ON' if value_json.undervoltage_detected else 'OFF' }}""#));
        assert!(s_uv.contains(r#""dev_cla":"problem""#));
        assert!(s_uv.contains(r#""uniq_id":"sysmqttd_test-host_undervoltage""#));
        assert!(!s_uv.contains("unit_of_meas"));
        assert!(!s_uv.contains("state_class"));

        // Throttled
        assert!(s_thr.contains(r#""name":"Throttled""#));
        assert!(s_thr.contains(r#""val_tpl":"{{ 'ON' if value_json.throttled else 'OFF' }}""#));
        assert!(s_thr.contains(r#""dev_cla":"problem""#));
        assert!(s_thr.contains(r#""uniq_id":"sysmqttd_test-host_throttled""#));
        assert!(!s_thr.contains("unit_of_meas"));
        assert!(!s_thr.contains("state_class"));
    }

    #[test]
    fn test_network_discovery_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "System Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };

        let ip_payload =
            DiscoveryPayload::new_ip_address("homeassistant", "test-host", device.clone());
        let mac_payload =
            DiscoveryPayload::new_mac_address("homeassistant", "test-host", device.clone());
        let rssi_payload = DiscoveryPayload::new_wifi_rssi("homeassistant", "test-host", device);

        let s_ip = serde_json::to_string(&ip_payload).unwrap();
        let s_mac = serde_json::to_string(&mac_payload).unwrap();
        let s_rssi = serde_json::to_string(&rssi_payload).unwrap();

        // IP Address
        assert!(s_ip.contains(r#""name":"IP Address""#));
        assert!(s_ip.contains(r#""val_tpl":"{{ value_json.ip_address }}""#));
        assert!(s_ip.contains(r#""uniq_id":"sysmqttd_test-host_ip_address""#));
        assert!(s_ip.contains(r#""ent_cat":"diagnostic""#));

        // MAC Address
        assert!(s_mac.contains(r#""name":"MAC Address""#));
        assert!(s_mac.contains(r#""val_tpl":"{{ value_json.mac_address }}""#));
        assert!(s_mac.contains(r#""uniq_id":"sysmqttd_test-host_mac_address""#));
        assert!(s_mac.contains(r#""ent_cat":"diagnostic""#));

        // Wi-Fi RSSI
        assert!(s_rssi.contains(r#""name":"Wi-Fi RSSI""#));
        assert!(s_rssi.contains(r#""val_tpl":"{{ value_json.wifi_rssi }}""#));
        assert!(s_rssi.contains(r#""unit_of_meas":"dBm""#));
        assert!(s_rssi.contains(r#""dev_cla":"signal_strength""#));
        assert!(s_rssi.contains(r#""state_class":"measurement""#));
        assert!(s_rssi.contains(r#""uniq_id":"sysmqttd_test-host_wifi_rssi""#));
        assert!(s_rssi.contains(r#""ent_cat":"diagnostic""#));
    }
}
