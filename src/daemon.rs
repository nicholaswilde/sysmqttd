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

    /// Spawn GPIO input monitoring/polling loop
    pub fn spawn_gpio_inputs_loop(&self, client: AsyncClient) {
        if self.config.gpio_inputs.is_empty() {
            return;
        }

        let hostname_clone = self.hostname.clone();
        let prefix_clone = self.config.mqtt_topic_prefix.clone();
        let mut listeners: Vec<crate::gpio_inputs::GpioInputListener> = self
            .config
            .gpio_inputs
            .iter()
            .map(|cfg| {
                crate::gpio_inputs::GpioInputListener::new(
                    cfg.pin,
                    cfg.name.clone(),
                    cfg.device_class.clone(),
                )
            })
            .collect();

        // Run setup and publish initial state for each listener
        for listener in &mut listeners {
            if let Err(e) = listener.setup() {
                eprintln!("Failed to setup GPIO pin {}: {}", listener.pin, e);
            } else if let Ok(val) = listener.read_value() {
                let state_topic = format!(
                    "{}/binary_sensor/sysmqttd_{}_pin{}/state",
                    prefix_clone, hostname_clone, listener.pin
                );
                let state_payload = if val == 1 { "ON" } else { "OFF" };
                let client_clone = client.clone();
                tokio::spawn(async move {
                    if let Err(e) = client_clone
                        .publish(&state_topic, QoS::AtLeastOnce, true, state_payload)
                        .await
                    {
                        eprintln!(
                            "GPIO initial state publication error for pin {}: {}",
                            state_topic, e
                        );
                    }
                });
            }
        }

        let verbose_clone = self.config.verbose;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(100));
            loop {
                interval.tick().await;
                for listener in &mut listeners {
                    match listener.check_transition() {
                        Ok(Some(val)) => {
                            let state_topic = format!(
                                "{}/binary_sensor/sysmqttd_{}_pin{}/state",
                                prefix_clone, hostname_clone, listener.pin
                            );
                            let state_payload = if val == 1 { "ON" } else { "OFF" };
                            if verbose_clone {
                                println!(
                                    "Publishing GPIO state: {} -> {}",
                                    state_topic, state_payload
                                );
                            }
                            if let Err(e) = client
                                .publish(&state_topic, QoS::AtLeastOnce, true, state_payload)
                                .await
                            {
                                eprintln!(
                                    "GPIO state publication error for pin {}: {}",
                                    listener.pin, e
                                );
                            }
                        }
                        Ok(None) => {}
                        Err(e) => {
                            eprintln!("Error reading GPIO pin {}: {}", listener.pin, e);
                        }
                    }
                }
            }
        });
    }

    /// Setup GPIO outputs (export, configure as "out", read/publish initial states)
    pub fn setup_gpio_outputs(&self, client: AsyncClient) {
        if self.config.gpio_outputs.is_empty() {
            return;
        }

        let hostname_clone = self.hostname.clone();
        let prefix_clone = self.config.mqtt_topic_prefix.clone();
        let controllers: Vec<crate::gpio_outputs::GpioOutputController> = self
            .config
            .gpio_outputs
            .iter()
            .map(|cfg| crate::gpio_outputs::GpioOutputController::new(cfg.pin, cfg.name.clone()))
            .collect();

        for controller in controllers {
            if let Err(e) = controller.setup() {
                eprintln!("Failed to setup GPIO output pin {}: {}", controller.pin, e);
            } else {
                // Read current state (could be 0 or 1) and publish
                let state_val = controller.read_value().unwrap_or(0);
                let state_topic = format!(
                    "{}/switch/sysmqttd_{}_pin{}/state",
                    prefix_clone, hostname_clone, controller.pin
                );
                let state_payload = if state_val == 1 { "ON" } else { "OFF" };
                let client_clone = client.clone();
                tokio::spawn(async move {
                    if let Err(e) = client_clone
                        .publish(&state_topic, QoS::AtLeastOnce, true, state_payload)
                        .await
                    {
                        eprintln!(
                            "GPIO output initial state publication error for pin {}: {}",
                            state_topic, e
                        );
                    }
                });
            }
        }
    }

    /// Spawn the non-blocking async telemetry polling loop
    pub fn spawn_telemetry_loop(&self, client: AsyncClient) {
        let hostname_clone = self.hostname.clone();
        let prefix_clone = self.config.mqtt_topic_prefix.clone();
        let interface_clone = self.config.net_interface.clone();

        let verbose_clone = self.config.verbose;
        tokio::spawn(async move {
            time::sleep(Duration::from_secs(5)).await;
            let mut collector = telemetry::TelemetryCollector::new();
            let state_topic = format!("{}/sensor/sysmqttd_{}/state", prefix_clone, hostname_clone);

            loop {
                let state = collector.collect(&interface_clone);
                match serde_json::to_vec(&state) {
                    Ok(payload) => {
                        if verbose_clone {
                            println!("Publishing telemetry state: {:?}", state);
                        }
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
            model: "System Monitor".to_string(),
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

        // 1.5. CPU Usage Discovery configuration
        let cpu_usage_payload = discovery::DiscoveryPayload::new_cpu_usage(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let cpu_usage_topic = format!(
            "{}/sensor/sysmqttd_{}_cpu_usage/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let cpu_usage_json = serde_json::to_vec(&cpu_usage_payload).unwrap();
        client
            .publish(cpu_usage_topic, QoS::AtLeastOnce, true, cpu_usage_json)
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

        // 2.1. RAM Used Discovery configuration
        let ram_used_payload = discovery::DiscoveryPayload::new_ram_used(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let ram_used_topic = format!(
            "{}/sensor/sysmqttd_{}_ram_used/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let ram_used_json = serde_json::to_vec(&ram_used_payload).unwrap();
        client
            .publish(ram_used_topic, QoS::AtLeastOnce, true, ram_used_json)
            .await?;

        // 2.2. RAM Free Discovery configuration
        let ram_free_payload = discovery::DiscoveryPayload::new_ram_free(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let ram_free_topic = format!(
            "{}/sensor/sysmqttd_{}_ram_free/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let ram_free_json = serde_json::to_vec(&ram_free_payload).unwrap();
        client
            .publish(ram_free_topic, QoS::AtLeastOnce, true, ram_free_json)
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

        // 3.1. Disk Used Discovery configuration
        let disk_used_payload = discovery::DiscoveryPayload::new_disk_used(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let disk_used_topic = format!(
            "{}/sensor/sysmqttd_{}_disk_used/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let disk_used_json = serde_json::to_vec(&disk_used_payload).unwrap();
        client
            .publish(disk_used_topic, QoS::AtLeastOnce, true, disk_used_json)
            .await?;

        // 3.2. Disk Free Discovery configuration
        let disk_free_payload = discovery::DiscoveryPayload::new_disk_free(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let disk_free_topic = format!(
            "{}/sensor/sysmqttd_{}_disk_free/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let disk_free_json = serde_json::to_vec(&disk_free_payload).unwrap();
        client
            .publish(disk_free_topic, QoS::AtLeastOnce, true, disk_free_json)
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
            device.clone(),
        );
        let load_15m_topic = format!(
            "{}/sensor/sysmqttd_{}_load_15m/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let load_15m_json = serde_json::to_vec(&load_15m_payload).unwrap();
        client
            .publish(load_15m_topic, QoS::AtLeastOnce, true, load_15m_json)
            .await?;

        // 7. Network RX Rate Discovery configuration
        let net_rx_payload = discovery::DiscoveryPayload::new_net_rx_rate(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let net_rx_topic = format!(
            "{}/sensor/sysmqttd_{}_net_rx_rate/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let net_rx_json = serde_json::to_vec(&net_rx_payload).unwrap();
        client
            .publish(net_rx_topic, QoS::AtLeastOnce, true, net_rx_json)
            .await?;

        // 8. Network TX Rate Discovery configuration
        let net_tx_payload = discovery::DiscoveryPayload::new_net_tx_rate(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let net_tx_topic = format!(
            "{}/sensor/sysmqttd_{}_net_tx_rate/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let net_tx_json = serde_json::to_vec(&net_tx_payload).unwrap();
        client
            .publish(net_tx_topic, QoS::AtLeastOnce, true, net_tx_json)
            .await?;

        // 8.5. System Uptime Discovery configuration
        let uptime_payload = discovery::DiscoveryPayload::new_uptime(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device,
        );
        let uptime_topic = format!(
            "{}/sensor/sysmqttd_{}_uptime/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let uptime_json = serde_json::to_vec(&uptime_payload).unwrap();
        client
            .publish(uptime_topic, QoS::AtLeastOnce, true, uptime_json)
            .await?;

        // 9. GPIO Inputs discovery configurations
        for pin_config in &self.config.gpio_inputs {
            let unique_id = format!("sysmqttd_{}_pin{}", self.hostname, pin_config.pin);
            let discovery_topic = format!(
                "{}/binary_sensor/sysmqttd_{}_pin{}/config",
                self.config.mqtt_topic_prefix, self.hostname, pin_config.pin
            );

            let payload = serde_json::json!({
                "name": pin_config.name,
                "state_topic": format!("{}/binary_sensor/sysmqttd_{}_pin{}/state", self.config.mqtt_topic_prefix, self.hostname, pin_config.pin),
                "unique_id": unique_id,
                "device_class": pin_config.device_class,
                "payload_on": "ON",
                "payload_off": "OFF",
                "availability_topic": format!("{}/sensor/sysmqttd_{}/availability", self.config.mqtt_topic_prefix, self.hostname),
            });
            let payload_bytes = serde_json::to_vec(&payload).unwrap();
            client
                .publish(discovery_topic, QoS::AtLeastOnce, true, payload_bytes)
                .await?;
        }

        // 10. GPIO Outputs discovery configurations
        for pin_config in &self.config.gpio_outputs {
            let unique_id = format!("sysmqttd_{}_pin{}", self.hostname, pin_config.pin);
            let discovery_topic = format!(
                "{}/switch/sysmqttd_{}_pin{}/config",
                self.config.mqtt_topic_prefix, self.hostname, pin_config.pin
            );

            let payload = serde_json::json!({
                "name": pin_config.name,
                "state_topic": format!("{}/switch/sysmqttd_{}_pin{}/state", self.config.mqtt_topic_prefix, self.hostname, pin_config.pin),
                "command_topic": format!("{}/switch/sysmqttd_{}_pin{}/set", self.config.mqtt_topic_prefix, self.hostname, pin_config.pin),
                "unique_id": unique_id,
                "payload_on": "ON",
                "payload_off": "OFF",
                "availability_topic": format!("{}/sensor/sysmqttd_{}/availability", self.config.mqtt_topic_prefix, self.hostname),
            });
            let payload_bytes = serde_json::to_vec(&payload).unwrap();
            client
                .publish(discovery_topic, QoS::AtLeastOnce, true, payload_bytes)
                .await?;
        }

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
        // Spawn GPIO Inputs Polling Loop
        self.spawn_gpio_inputs_loop(client.clone());
        // Setup GPIO Outputs
        self.setup_gpio_outputs(client.clone());

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
                        Ok(Event::Incoming(incoming)) => {
                            if self.config.verbose {
                                println!("MQTT Incoming Packet: {:?}", incoming);
                            }
                            match incoming {
                                Packet::ConnAck(connack) => {
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
                                    // Subscribe to GPIO output command topics
                                    for pin_config in &self.config.gpio_outputs {
                                         let cmd_topic = format!(
                                             "{}/switch/sysmqttd_{}_pin{}/set",
                                             self.config.mqtt_topic_prefix, self.hostname, pin_config.pin
                                         );
                                         if let Err(e) = client.subscribe(&cmd_topic, QoS::AtLeastOnce).await {
                                             eprintln!("Failed to subscribe to GPIO command topic {}: {}", cmd_topic, e);
                                         }
                                    }
                                    // Subscribe to remote command topic
                                    let remote_cmd_topic = format!(
                                        "{}/sensor/sysmqttd_{}/command",
                                        self.config.mqtt_topic_prefix, self.hostname
                                    );
                                    if let Err(e) = client.subscribe(&remote_cmd_topic, QoS::AtLeastOnce).await {
                                        eprintln!("Failed to subscribe to remote commands topic {}: {}", remote_cmd_topic, e);
                                    }
                                }
                                Packet::Publish(publish) => {
                                    let prefix = &self.config.mqtt_topic_prefix;
                                    let hostname = &self.hostname;

                                    // Check if this publish is for our remote command topic
                                    let global_cmd_topic = format!("{}/sensor/sysmqttd_{}/command", prefix, hostname);
                                    if publish.topic == global_cmd_topic {
                                        let payload_str = String::from_utf8_lossy(&publish.payload);
                                        match payload_str.trim().parse::<crate::commands::RemoteAction>() {
                                            Ok(action) => {
                                                println!("Executing whitelisted remote command: {:?}", action);
                                                if let Err(e) = action.execute() {
                                                    eprintln!("Error executing remote command {:?}: {}", action, e);
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!("Ignoring unauthorized or malformed remote command payload: {}", e);
                                            }
                                        }
                                    }

                                    // Check if this publish is for one of our GPIO output command topics
                                    for pin_config in &self.config.gpio_outputs {
                                        let cmd_topic = format!(
                                            "{}/switch/sysmqttd_{}_pin{}/set",
                                            prefix, hostname, pin_config.pin
                                        );
                                        if publish.topic == cmd_topic {
                                            let payload_str = String::from_utf8_lossy(&publish.payload).trim().to_uppercase();
                                            let val = match payload_str.as_str() {
                                                "ON" => Some(1),
                                                "OFF" => Some(0),
                                                _ => {
                                                    eprintln!("Unknown GPIO command payload: {}", payload_str);
                                                    None
                                                }
                                            };
                                            if let Some(v) = val {
                                                let controller = crate::gpio_outputs::GpioOutputController::new(
                                                    pin_config.pin,
                                                    pin_config.name.clone(),
                                                );
                                                if let Err(e) = controller.write_value(v) {
                                                    eprintln!("Failed to write GPIO output pin {}: {}", pin_config.pin, e);
                                                } else {
                                                    if self.config.verbose {
                                                        println!("Set GPIO output pin {} to {}", pin_config.pin, v);
                                                    }
                                                    // Publish confirmed state back
                                                    let state_topic = format!(
                                                        "{}/switch/sysmqttd_{}_pin{}/state",
                                                        prefix, hostname, pin_config.pin
                                                    );
                                                    let confirmed_payload = if v == 1 { "ON" } else { "OFF" };
                                                    let client_clone = client.clone();
                                                    tokio::spawn(async move {
                                                        if let Err(e) = client_clone.publish(state_topic, QoS::AtLeastOnce, true, confirmed_payload).await {
                                                            eprintln!("Failed to publish state confirmation: {}", e);
                                                        }
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        Ok(Event::Outgoing(outgoing)) => {
                            if self.config.verbose {
                                println!("MQTT Outgoing Packet: {:?}", outgoing);
                            }
                        }
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
    use crate::gpio_inputs::GpioInputConfig;

    #[test]
    fn test_daemon_mqtt_options_mapping() {
        let config = Config {
            mqtt_host: "10.0.0.5".to_string(),
            mqtt_port: 1883,
            mqtt_user: Some("my_user".to_string()),
            mqtt_password: Some("my_pass".to_string()),
            mqtt_topic_prefix: "ha_home".to_string(),
            net_interface: "wlan0".to_string(),
            gpio_inputs: vec![],
            gpio_outputs: vec![],
            verbose: false,
        };
        let daemon = Daemon::new(config, "pi-zero".to_string());

        let options = daemon.get_mqtt_options();
        assert_eq!(options.broker_address(), ("10.0.0.5".to_string(), 1883));

        let client_id = format!("sysmqttd_{}", "pi-zero");
        assert_eq!(options.client_id(), client_id);
    }

    #[test]
    fn test_daemon_with_gpio_inputs_config() {
        let config = Config {
            mqtt_host: "10.0.0.5".to_string(),
            mqtt_port: 1883,
            mqtt_user: None,
            mqtt_password: None,
            mqtt_topic_prefix: "ha_home".to_string(),
            net_interface: "wlan0".to_string(),
            gpio_inputs: vec![GpioInputConfig {
                pin: 23,
                name: "Front Door".to_string(),
                device_class: Some("door".to_string()),
            }],
            gpio_outputs: vec![],
            verbose: false,
        };
        let daemon = Daemon::new(config, "pi-zero".to_string());
        assert_eq!(daemon.config.gpio_inputs.len(), 1);
        assert_eq!(daemon.config.gpio_inputs[0].pin, 23);
        assert_eq!(daemon.config.gpio_inputs[0].name, "Front Door");
    }

    #[test]
    fn test_daemon_with_gpio_outputs_config() {
        let config = Config {
            mqtt_host: "10.0.0.5".to_string(),
            mqtt_port: 1883,
            mqtt_user: None,
            mqtt_password: None,
            mqtt_topic_prefix: "ha_home".to_string(),
            net_interface: "wlan0".to_string(),
            gpio_inputs: vec![],
            gpio_outputs: vec![crate::gpio_outputs::GpioOutputConfig {
                pin: 24,
                name: "Mock Switch".to_string(),
            }],
            verbose: false,
        };
        let daemon = Daemon::new(config, "pi-zero".to_string());
        assert_eq!(daemon.config.gpio_outputs.len(), 1);
        assert_eq!(daemon.config.gpio_outputs[0].pin, 24);
        assert_eq!(daemon.config.gpio_outputs[0].name, "Mock Switch");
    }
}
