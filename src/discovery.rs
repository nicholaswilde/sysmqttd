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
            model: "Raspberry Pi Zero W Monitor".to_string(),
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
        assert!(serialized.contains(r#""dev":{"ids":["sysmqttd_test-host"],"name":"sysmqttd test-host","mdl":"Raspberry Pi Zero W Monitor","mf":"sysmqttd"}"#));
    }

    #[test]
    fn test_load_avg_serialization() {
        let device = DeviceInfo {
            identifiers: vec!["sysmqttd_test-host".to_string()],
            name: "sysmqttd test-host".to_string(),
            model: "Raspberry Pi Zero W Monitor".to_string(),
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
}
