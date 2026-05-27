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
    #[serde(rename = "unit_of_meas")]
    pub unit_of_measurement: String,
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,
    #[serde(rename = "state_class")]
    pub state_class: String,
    #[serde(rename = "uniq_id")]
    pub unique_id: String,
    #[serde(rename = "dev")]
    pub device: DeviceInfo,
}

impl DiscoveryPayload {
    pub fn new_cpu_temp(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "CPU Temperature".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.cpu_temperature }}".to_string(),
            unit_of_measurement: "°C".to_string(),
            device_class: Some("temperature".to_string()),
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_cpu_temp", hostname),
            device,
        }
    }

    pub fn new_cpu_usage(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "CPU Usage".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.cpu_usage }}".to_string(),
            unit_of_measurement: "%".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_cpu_usage", hostname),
            device,
        }
    }

    pub fn new_ram_usage(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "RAM Usage".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.ram_usage }}".to_string(),
            unit_of_measurement: "%".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_ram_usage", hostname),
            device,
        }
    }

    pub fn new_ram_used(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "RAM Used".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.ram_used_mb }}".to_string(),
            unit_of_measurement: "MB".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_ram_used", hostname),
            device,
        }
    }

    pub fn new_ram_free(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "RAM Free".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.ram_free_mb }}".to_string(),
            unit_of_measurement: "MB".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_ram_free", hostname),
            device,
        }
    }

    pub fn new_disk_usage(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Disk Storage Utilization".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.disk_usage }}".to_string(),
            unit_of_measurement: "%".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_disk_usage", hostname),
            device,
        }
    }

    pub fn new_disk_used(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Disk Used".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.disk_used_gb }}".to_string(),
            unit_of_measurement: "GB".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_disk_used", hostname),
            device,
        }
    }

    pub fn new_disk_free(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Disk Free".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.disk_free_gb }}".to_string(),
            unit_of_measurement: "GB".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_disk_free", hostname),
            device,
        }
    }

    pub fn new_load_1m(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Load Avg (1m)".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.load_1m }}".to_string(),
            unit_of_measurement: "".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_load_1m", hostname),
            device,
        }
    }

    pub fn new_load_5m(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Load Avg (5m)".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.load_5m }}".to_string(),
            unit_of_measurement: "".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_load_5m", hostname),
            device,
        }
    }

    pub fn new_load_15m(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Load Avg (15m)".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.load_15m }}".to_string(),
            unit_of_measurement: "".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_load_15m", hostname),
            device,
        }
    }

    pub fn new_net_rx_rate(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Network RX Rate".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.net_rx_rate }}".to_string(),
            unit_of_measurement: "kB/s".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_net_rx_rate", hostname),
            device,
        }
    }

    pub fn new_net_tx_rate(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Network TX Rate".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.net_tx_rate }}".to_string(),
            unit_of_measurement: "kB/s".to_string(),
            device_class: None,
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_net_tx_rate", hostname),
            device,
        }
    }

    pub fn new_uptime(prefix: &str, hostname: &str, device: DeviceInfo) -> Self {
        DiscoveryPayload {
            name: "Uptime".to_string(),
            state_topic: format!("{}/sensor/sysmqttd_{}/state", prefix, hostname),
            availability_topic: format!("{}/sensor/sysmqttd_{}/availability", prefix, hostname),
            value_template: "{{ value_json.uptime_seconds }}".to_string(),
            unit_of_measurement: "s".to_string(),
            device_class: Some("duration".to_string()),
            state_class: "measurement".to_string(),
            unique_id: format!("sysmqttd_{}_uptime", hostname),
            device,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_cpu_temp_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "System Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };
        let payload = DiscoveryPayload::new_cpu_temp("homeassistant", "test-host", device);
        let serialized = serde_json::to_string(&payload).unwrap();

        // Assertions verifying exact keys and structure
        assert!(serialized.contains(r#""name":"CPU Temperature""#));
        assert!(serialized.contains(r#""stat_t":"homeassistant/sensor/sysmqttd_test-host/state""#));
        assert!(serialized
            .contains(r#""avty_t":"homeassistant/sensor/sysmqttd_test-host/availability""#));
        assert!(serialized.contains(r#""val_tpl":"{{ value_json.cpu_temperature }}""#));
        assert!(serialized.contains(r#""unit_of_meas":"°C""#));
        assert!(serialized.contains(r#""dev_cla":"temperature""#));
        assert!(serialized.contains(r#""state_class":"measurement""#));
        assert!(serialized.contains(r#""uniq_id":"sysmqttd_test-host_cpu_temp""#));
        assert!(serialized.contains(r#""dev":{"ids":["sysmqttd_test-host"],"name":"sysmqttd test-host","mdl":"System Monitor","mf":"sysmqttd"}"#));
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
}
