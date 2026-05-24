use crate::config::Config;
use crate::discovery;
use crate::telemetry;
use rumqttc::{AsyncClient, Event, LastWill, MqttOptions, Packet, QoS};
use std::time::Duration;
use tokio::time;

pub struct Daemon {
    pub config: Config,
    pub hostname: String,
}

impl Daemon {
    pub fn new(config: Config, hostname: String) -> Self {
        Daemon { config, hostname }
    }

    /// Set up MqttOptions for client
    pub fn get_mqtt_options(&self) -> MqttOptions {
        let client_id = format!("sysmqttd_{}", self.hostname);
        let mut mqttoptions =
            MqttOptions::new(client_id, &self.config.mqtt_host, self.config.mqtt_port);
        mqttoptions.set_keep_alive(Duration::from_secs(30));

        // Define Last Will
        let availability_topic = format!(
            "{}/sensor/sysmqttd_{}/availability",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let last_will = LastWill::new(availability_topic, "offline", QoS::AtLeastOnce, true);
        mqttoptions.set_last_will(last_will);

        if let (Some(user), Some(pass)) = (&self.config.mqtt_user, &self.config.mqtt_password) {
            mqttoptions.set_credentials(user, pass);
        }
        mqttoptions
    }

    /// Spawn service status monitoring loop
    pub fn spawn_service_status_loop(&self, client: AsyncClient) {
        let hostname_clone = self.hostname.clone();
        let prefix_clone = self.config.mqtt_topic_prefix.clone();
        // Load services from env var or default empty list
        let services = crate::service_status::parse_monitored_services();
        let monitor = crate::service_status::ServiceStatusMonitor::new(
            services,
            prefix_clone,
            hostname_clone,
        );

        tokio::spawn(async move {
            // Initial publish
            if let Err(e) = monitor.publish_statuses(&client).await {
                eprintln!("Service status publish error: {}", e);
            }
            // Periodic check every 30 seconds
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                if let Err(e) = monitor.publish_statuses(&client).await {
                    eprintln!("Service status publish error: {}", e);
                }
            }
        });
    }

    /// Spawn the non-blocking async telemetry polling loop
    pub fn spawn_telemetry_loop(&self, client: AsyncClient) {
        let hostname_clone = self.hostname.clone();
        let prefix_clone = self.config.mqtt_topic_prefix.clone();

        tokio::spawn(async move {
            time::sleep(Duration::from_secs(5)).await;
            let mut collector = telemetry::TelemetryCollector::new();
            let state_topic = format!("{}/sensor/sysmqttd_{}/state", prefix_clone, hostname_clone);

            loop {
                let state = collector.collect();
                match serde_json::to_vec(&state) {
                    Ok(payload) => {
                        println!("Publishing telemetry state: {:?}", state);
                        if let Err(e) = client
                            .publish(&state_topic, QoS::AtLeastOnce, false, payload)
                            .await
                        {
                            eprintln!("Telemetry publication error: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to serialize telemetry state payload: {}", e);
                    }
                }
                time::sleep(Duration::from_secs(60)).await;
            }
        });
    }

    /// Publish HA discovery payloads
    pub async fn publish_discovery(
        &self,
        client: &AsyncClient,
    ) -> Result<(), rumqttc::ClientError> {
        let device = discovery::DeviceInfo {
            identifiers: vec![format!("sysmqttd_{}", self.hostname)],
            name: format!("sysmqttd {}", self.hostname),
            model: "Raspberry Pi Zero W Monitor".to_string(),
            manufacturer: "sysmqttd".to_string(),
        };

        // 1. CPU Temperature Discovery configuration
        let cpu_payload = discovery::DiscoveryPayload::new_cpu_temp(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let cpu_topic = format!(
            "{}/sensor/sysmqttd_{}_cpu_temp/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let cpu_json = serde_json::to_vec(&cpu_payload).unwrap();
        client
            .publish(cpu_topic, QoS::AtLeastOnce, true, cpu_json)
            .await?;

        // 2. RAM Usage Discovery configuration
        let ram_payload = discovery::DiscoveryPayload::new_ram_usage(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let ram_topic = format!(
            "{}/sensor/sysmqttd_{}_ram_usage/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let ram_json = serde_json::to_vec(&ram_payload).unwrap();
        client
            .publish(ram_topic, QoS::AtLeastOnce, true, ram_json)
            .await?;

        // 3. Disk Usage Discovery configuration
        let disk_payload = discovery::DiscoveryPayload::new_disk_usage(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let disk_topic = format!(
            "{}/sensor/sysmqttd_{}_disk_usage/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let disk_json = serde_json::to_vec(&disk_payload).unwrap();
        client
            .publish(disk_topic, QoS::AtLeastOnce, true, disk_json)
            .await?;

        // 4. Load Avg (1m) Discovery configuration
        let load_1m_payload = discovery::DiscoveryPayload::new_load_1m(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let load_1m_topic = format!(
            "{}/sensor/sysmqttd_{}_load_1m/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let load_1m_json = serde_json::to_vec(&load_1m_payload).unwrap();
        client
            .publish(load_1m_topic, QoS::AtLeastOnce, true, load_1m_json)
            .await?;

        // 5. Load Avg (5m) Discovery configuration
        let load_5m_payload = discovery::DiscoveryPayload::new_load_5m(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let load_5m_topic = format!(
            "{}/sensor/sysmqttd_{}_load_5m/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let load_5m_json = serde_json::to_vec(&load_5m_payload).unwrap();
        client
            .publish(load_5m_topic, QoS::AtLeastOnce, true, load_5m_json)
            .await?;

        // 6. Load Avg (15m) Discovery configuration
        let load_15m_payload = discovery::DiscoveryPayload::new_load_15m(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device,
        );
        let load_15m_topic = format!(
            "{}/sensor/sysmqttd_{}_load_15m/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let load_15m_json = serde_json::to_vec(&load_15m_payload).unwrap();
        client
            .publish(load_15m_topic, QoS::AtLeastOnce, true, load_15m_json)
            .await?;

        println!("Published Home Assistant MQTT Discovery configs successfully.");
        Ok(())
    }

    /// Run the main daemon loop with support for a clean shutdown signal
    pub async fn run_with_shutdown(
        self,
        mut shutdown_rx: tokio::sync::oneshot::Receiver<()>,
    ) -> Result<(), String> {
        let mqttoptions = self.get_mqtt_options();
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // Spawn Telemetry Loop
        self.spawn_telemetry_loop(client.clone());
        // Spawn Service Status Loop
        self.spawn_service_status_loop(client.clone());

        println!("Connecting to MQTT broker...");
        loop {
            tokio::select! {
                _ = &mut shutdown_rx => {
                    println!("Shutdown signal received. Exiting daemon event loop.");
                    // Publish graceful offline message
                    let availability_topic = format!(
                        "{}/sensor/sysmqttd_{}/availability",
                        self.config.mqtt_topic_prefix, self.hostname
                    );
                    if let Err(e) = client.publish(&availability_topic, QoS::AtLeastOnce, true, "offline").await {
                        eprintln!("Failed to publish graceful offline availability state: {}", e);
                    }
                    break;
                }
                notification = eventloop.poll() => {
                    match notification {
                        Ok(Event::Incoming(Packet::ConnAck(connack))) => {
                            println!("Successfully connected to MQTT broker! ConnAck: {:?}", connack);
                            // Publish Birth Message
                            let availability_topic = format!(
                                "{}/sensor/sysmqttd_{}/availability",
                                self.config.mqtt_topic_prefix, self.hostname
                            );
                            if let Err(e) = client.publish(&availability_topic, QoS::AtLeastOnce, true, "online").await {
                                eprintln!("Failed to publish online availability state: {}", e);
                            }
                            if let Err(e) = self.publish_discovery(&client).await {
                                eprintln!("Failed to publish Home Assistant discovery configurations: {}", e);
                            }
                        }
                        Ok(Event::Incoming(_incoming)) => {}
                        Ok(Event::Outgoing(_outgoing)) => {}
                        Err(e) => {
                            eprintln!("MQTT EventLoop Error: {}. Retrying in 5 seconds...", e);
                            time::sleep(Duration::from_secs(5)).await;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Run the main daemon loop (blocks indefinitely)
    pub async fn run(self) -> Result<(), String> {
        let (_tx, rx) = tokio::sync::oneshot::channel();
        self.run_with_shutdown(rx).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daemon_mqtt_options_mapping() {
        let config = Config {
            mqtt_host: "10.0.0.5".to_string(),
            mqtt_port: 1883,
            mqtt_user: Some("my_user".to_string()),
            mqtt_password: Some("my_pass".to_string()),
            mqtt_topic_prefix: "ha_home".to_string(),
        };
        let daemon = Daemon::new(config, "pi-zero".to_string());

        let options = daemon.get_mqtt_options();
        assert_eq!(options.broker_address(), ("10.0.0.5".to_string(), 1883));

        let client_id = format!("sysmqttd_{}", "pi-zero");
        assert_eq!(options.client_id(), client_id);
    }
}
