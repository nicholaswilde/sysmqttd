use rumqttc::{AsyncClient, QoS};
use std::process::Command;

/// Monitors systemd service statuses and publishes binary sensor states to MQTT.
pub struct ServiceStatusMonitor {
    /// List of service names to monitor (e.g., ["docker", "nginx"]).
    services: Vec<String>,
    /// MQTT topic prefix from configuration.
    topic_prefix: String,
    /// Hostname of the device (used in topic naming).
    hostname: String,
}

impl ServiceStatusMonitor {
    pub fn new(services: Vec<String>, topic_prefix: String, hostname: String) -> Self {
        Self {
            services,
            topic_prefix,
            hostname,
        }
    }

    /// Checks `systemctl is-active <service>` for each service.
    /// Returns a vector of (service_name, is_active) tuples.
    fn check_statuses(&self) -> Vec<(String, bool)> {
        self.services
            .iter()
            .map(|svc| {
                let output = Command::new("systemctl").arg("is-active").arg(svc).output();
                let active = match output {
                    Ok(o) => o.stdout.starts_with(b"active"),
                    Err(_) => false,
                };
                (svc.clone(), active)
            })
            .collect()
    }

    /// Publishes binary sensor payloads for each monitored service.
    /// Uses Home Assistant discovery format.
    pub async fn publish_statuses(&self, client: &AsyncClient) -> Result<(), rumqttc::ClientError> {
        for (svc, active) in self.check_statuses() {
            let device_class = "connectivity"; // `online`/`offline` binary sensor
            let state = if active { "on" } else { "off" };
            let unique_id = format!("sysmqttd_{}_service_{}", self.hostname, svc);
            let discovery_topic = format!(
                "{}/binary_sensor/sysmqttd_{}_service_{}/config",
                self.topic_prefix, self.hostname, svc
            );
            let payload = serde_json::json!({
                "name": format!("{} Service", svc),
                "state_topic": format!("{}/binary_sensor/sysmqttd_{}/service_{}/state", self.topic_prefix, self.hostname, svc),
                "unique_id": unique_id,
                "device_class": device_class,
                "payload_on": "on",
                "payload_off": "off",
                "availability_topic": format!("{}/sensor/sysmqttd_{}/availability", self.topic_prefix, self.hostname),
            });
            let payload_bytes = serde_json::to_vec(&payload).unwrap();
            client
                .publish(&discovery_topic, QoS::AtLeastOnce, true, payload_bytes)
                .await?;

            // Publish current state
            let state_topic = format!(
                "{}/binary_sensor/sysmqttd_{}/service_{}/state",
                self.topic_prefix, self.hostname, svc
            );
            client
                .publish(&state_topic, QoS::AtLeastOnce, true, state)
                .await?;
        }
        Ok(())
    }
}

/// Helper to parse a comma‑separated list of services from the `MONITORED_SERVICES` environment variable.
pub fn parse_monitored_services() -> Vec<String> {
    std::env::var("MONITORED_SERVICES")
        .ok()
        .map(|s| {
            s.split(',')
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monitored_services() {
        std::env::set_var("MONITORED_SERVICES", "docker, nginx,, ssh ");
        let services = parse_monitored_services();
        assert_eq!(services, vec!["docker", "nginx", "ssh"]);

        std::env::remove_var("MONITORED_SERVICES");
        let services_empty = parse_monitored_services();
        assert!(services_empty.is_empty());
    }

    #[test]
    fn test_monitor_new() {
        let monitor = ServiceStatusMonitor::new(
            vec!["docker".to_string()],
            "homeassistant".to_string(),
            "test-host".to_string(),
        );
        assert_eq!(monitor.services, vec!["docker"]);
        assert_eq!(monitor.topic_prefix, "homeassistant");
        assert_eq!(monitor.hostname, "test-host");
    }

    #[test]
    fn test_check_statuses_fallback() {
        let monitor = ServiceStatusMonitor::new(
            vec!["non_existent_service_12345".to_string()],
            "homeassistant".to_string(),
            "test-host".to_string(),
        );
        let statuses = monitor.check_statuses();
        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].0, "non_existent_service_12345");
        // Because systemctl will report inactive (or not exist), active should be false
        assert!(!statuses[0].1);
    }
}
