use crate::config::Config;
use crate::discovery;
use crate::telemetry;
use rumqttc::{AsyncClient, Event, LastWill, MqttOptions, Packet, QoS};
use std::time::Duration;
use tokio::time;

#[derive(Clone)]
pub struct Daemon {
    pub config: Config,
    pub hostname: String,
    pub gpio_base_path: std::path::PathBuf,
    pub sysfs_root: std::path::PathBuf,
}

impl Daemon {
    pub fn new(config: Config, hostname: String) -> Self {
        Daemon {
            config,
            hostname,
            gpio_base_path: std::path::PathBuf::from("/sys/class/gpio"),
            sysfs_root: std::path::PathBuf::from("/"),
        }
    }

    pub fn with_gpio_base_path(mut self, path: std::path::PathBuf) -> Self {
        self.gpio_base_path = path;
        self
    }

    pub fn with_sysfs_root(mut self, path: std::path::PathBuf) -> Self {
        self.sysfs_root = path;
        self
    }

    /// Set up MqttOptions for client
    pub fn get_mqtt_options(&self) -> Result<MqttOptions, String> {
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

        if self.config.use_tls {
            let tls_config = self.get_tls_config()?;
            mqttoptions.set_transport(rumqttc::Transport::tls_with_config(tls_config.into()));
        }

        Ok(mqttoptions)
    }

    fn get_tls_config(&self) -> Result<tokio_rustls::rustls::ClientConfig, String> {
        let mut root_cert_store = tokio_rustls::rustls::RootCertStore::empty();

        if let Some(ca_path) = &self.config.ca_cert_path {
            let file = std::fs::File::open(ca_path)
                .map_err(|e| format!("Failed to open CA certificate file '{}': {}", ca_path, e))?;
            let mut reader = std::io::BufReader::new(file);
            let mut certs_loaded = 0;
            for cert_result in rustls_pemfile::certs(&mut reader) {
                let cert = cert_result
                    .map_err(|e| format!("Failed to parse CA certificate PEM: {}", e))?;
                root_cert_store
                    .add(cert)
                    .map_err(|e| format!("Failed to add CA certificate to store: {}", e))?;
                certs_loaded += 1;
            }
            if certs_loaded == 0 {
                return Err(format!("No certificates found in CA path '{}'", ca_path));
            }
            println!(
                "Loaded {} custom CA certificate(s) from {}",
                certs_loaded, ca_path
            );
        } else {
            // Load native platform certs
            let certs = rustls_native_certs::load_native_certs()
                .map_err(|e| format!("Failed to load native root certificates: {}", e))?;
            let mut certs_loaded = 0;
            for cert in certs {
                if root_cert_store.add(cert).is_ok() {
                    certs_loaded += 1;
                }
            }
            if certs_loaded == 0 {
                return Err("Failed to load any native root certificates".to_string());
            }
        }

        let client_config = tokio_rustls::rustls::ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();

        Ok(client_config)
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
                crate::gpio_inputs::GpioInputListener::with_base_path(
                    cfg.pin,
                    cfg.name.clone(),
                    cfg.device_class.clone(),
                    self.gpio_base_path.clone(),
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
            .map(|cfg| {
                crate::gpio_outputs::GpioOutputController::with_base_path(
                    cfg.pin,
                    cfg.name.clone(),
                    self.gpio_base_path.clone(),
                )
            })
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
    pub fn spawn_telemetry_loop(
        &self,
        client: AsyncClient,
        mut interval_rx: tokio::sync::watch::Receiver<u64>,
    ) {
        let hostname_clone = self.hostname.clone();
        let prefix_clone = self.config.mqtt_topic_prefix.clone();
        let interface_clone = self.config.net_interface.clone();

        let unit_clone = self.config.temperature_unit.clone();
        let verbose_clone = self.config.verbose;
        let sd_threshold_clone = self.config.sd_alert_threshold;
        let no_fan_clone = self.config.no_fan;
        let sysfs_root_clone = self.sysfs_root.clone();
        tokio::spawn(async move {
            time::sleep(Duration::from_secs(5)).await;
            let mut collector = telemetry::TelemetryCollector::with_sysfs_root(sysfs_root_clone);
            collector.temperature_unit = unit_clone;
            collector.sd_alert_threshold = sd_threshold_clone;
            collector.no_fan = no_fan_clone;
            let state_topic = format!("{}/sensor/sysmqttd_{}/state", prefix_clone, hostname_clone);

            let mut current_interval = Duration::from_secs(*interval_rx.borrow());

            // First publish immediately after startup delay
            let state = collector.collect(&interface_clone);
            crate::logging::set_quiet_logging(state.sd_space_alert);

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

            let sleep = time::sleep(current_interval);
            tokio::pin!(sleep);

            loop {
                tokio::select! {
                    _ = &mut sleep => {
                        let state = collector.collect(&interface_clone);
                        crate::logging::set_quiet_logging(state.sd_space_alert);

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
                        current_interval = Duration::from_secs(*interval_rx.borrow());
                        sleep.as_mut().reset(time::Instant::now() + current_interval);
                    }
                    changed = interval_rx.changed() => {
                        if changed.is_ok() {
                            let new_val = *interval_rx.borrow();
                            if verbose_clone {
                                println!("Telemetry loop received updated interval: {}s", new_val);
                            }
                            current_interval = Duration::from_secs(new_val);
                            sleep.as_mut().reset(time::Instant::now() + current_interval);
                        } else {
                            break;
                        }
                    }
                }
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
            &self.config.temperature_unit,
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
            device.clone(),
        );
        let uptime_topic = format!(
            "{}/sensor/sysmqttd_{}_uptime/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let uptime_json = serde_json::to_vec(&uptime_payload).unwrap();
        client
            .publish(uptime_topic, QoS::AtLeastOnce, true, uptime_json)
            .await?;

        // 8.6. Under-voltage Discovery configuration
        let undervoltage_payload = discovery::DiscoveryPayload::new_undervoltage(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let undervoltage_topic = format!(
            "{}/binary_sensor/sysmqttd_{}_undervoltage/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let undervoltage_json = serde_json::to_vec(&undervoltage_payload).unwrap();
        client
            .publish(
                undervoltage_topic,
                QoS::AtLeastOnce,
                true,
                undervoltage_json,
            )
            .await?;

        // 8.7. Throttled Discovery configuration
        let throttled_payload = discovery::DiscoveryPayload::new_throttled(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let throttled_topic = format!(
            "{}/binary_sensor/sysmqttd_{}_throttled/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let throttled_json = serde_json::to_vec(&throttled_payload).unwrap();
        client
            .publish(throttled_topic, QoS::AtLeastOnce, true, throttled_json)
            .await?;

        // 8.7.5. SD Card Space Alert Discovery configuration
        let sd_space_alert_payload = discovery::DiscoveryPayload::new_sd_space_alert(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let sd_space_alert_topic = format!(
            "{}/binary_sensor/sysmqttd_{}_sd_space_alert/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let sd_space_alert_json = serde_json::to_vec(&sd_space_alert_payload).unwrap();
        client
            .publish(
                sd_space_alert_topic,
                QoS::AtLeastOnce,
                true,
                sd_space_alert_json,
            )
            .await?;

        // 8.8. IP Address Discovery configuration
        let ip_payload = discovery::DiscoveryPayload::new_ip_address(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let ip_topic = format!(
            "{}/sensor/sysmqttd_{}_ip_address/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let ip_json = serde_json::to_vec(&ip_payload).unwrap();
        client
            .publish(ip_topic, QoS::AtLeastOnce, true, ip_json)
            .await?;

        // 8.9. MAC Address Discovery configuration
        let mac_payload = discovery::DiscoveryPayload::new_mac_address(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let mac_topic = format!(
            "{}/sensor/sysmqttd_{}_mac_address/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let mac_json = serde_json::to_vec(&mac_payload).unwrap();
        client
            .publish(mac_topic, QoS::AtLeastOnce, true, mac_json)
            .await?;

        // 8.10. Wi-Fi RSSI Discovery configuration
        let rssi_payload = discovery::DiscoveryPayload::new_wifi_rssi(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let rssi_topic = format!(
            "{}/sensor/sysmqttd_{}_wifi_rssi/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let rssi_json = serde_json::to_vec(&rssi_payload).unwrap();
        client
            .publish(rssi_topic, QoS::AtLeastOnce, true, rssi_json)
            .await?;

        // 8.11. Upgradable Packages Discovery configuration
        let pkgs_payload = discovery::DiscoveryPayload::new_upgradable_packages(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let pkgs_topic = format!(
            "{}/sensor/sysmqttd_{}_upgradable_packages/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let pkgs_json = serde_json::to_vec(&pkgs_payload).unwrap();
        client
            .publish(pkgs_topic, QoS::AtLeastOnce, true, pkgs_json)
            .await?;

        // 8.12. Top Process Discovery configuration
        let top_proc_payload = discovery::DiscoveryPayload::new_top_process(
            &self.config.mqtt_topic_prefix,
            &self.hostname,
            device.clone(),
        );
        let top_proc_topic = format!(
            "{}/sensor/sysmqttd_{}_top_process/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let top_proc_json = serde_json::to_vec(&top_proc_payload).unwrap();
        client
            .publish(top_proc_topic, QoS::AtLeastOnce, true, top_proc_json)
            .await?;

        // 8.12.5. Fan Speed Discovery configuration
        if !self.config.no_fan {
            let mut collector =
                telemetry::TelemetryCollector::with_sysfs_root(self.sysfs_root.clone());
            collector.no_fan = self.config.no_fan;
            let fan_speeds = collector.read_fan_speeds();
            for (fan_id, _) in fan_speeds {
                let fan_name = if let Some(index) = fan_id.strip_prefix("fan_") {
                    format!("Fan {} Speed", index)
                } else {
                    format!("{} Speed", fan_id)
                };

                let fan_payload = discovery::DiscoveryPayload::new_fan_speed(
                    &self.config.mqtt_topic_prefix,
                    &self.hostname,
                    &fan_id,
                    &fan_name,
                    device.clone(),
                );
                let fan_topic = format!(
                    "{}/sensor/sysmqttd_{}_{}/config",
                    self.config.mqtt_topic_prefix, self.hostname, fan_id
                );
                let fan_json = serde_json::to_vec(&fan_payload).unwrap();
                client
                    .publish(fan_topic, QoS::AtLeastOnce, true, fan_json)
                    .await?;
            }
        }

        // 8.13. Reboot Button Discovery configuration
        let reboot_topic = format!(
            "{}/button/sysmqttd_{}_reboot/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let reboot_payload = serde_json::json!({
            "name": "Reboot",
            "command_topic": format!("{}/sensor/sysmqttd_{}/command", self.config.mqtt_topic_prefix, self.hostname),
            "payload_press": "reboot",
            "unique_id": format!("sysmqttd_{}_reboot", self.hostname),
            "device": device,
            "availability_topic": format!("{}/sensor/sysmqttd_{}/availability", self.config.mqtt_topic_prefix, self.hostname),
        });
        let reboot_json = serde_json::to_vec(&reboot_payload).unwrap();
        client
            .publish(reboot_topic, QoS::AtLeastOnce, true, reboot_json)
            .await?;

        // 8.14. Shutdown Button Discovery configuration
        let shutdown_topic = format!(
            "{}/button/sysmqttd_{}_shutdown/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let shutdown_payload = serde_json::json!({
            "name": "Shutdown",
            "command_topic": format!("{}/sensor/sysmqttd_{}/command", self.config.mqtt_topic_prefix, self.hostname),
            "payload_press": "shutdown",
            "unique_id": format!("sysmqttd_{}_shutdown", self.hostname),
            "device": device,
            "availability_topic": format!("{}/sensor/sysmqttd_{}/availability", self.config.mqtt_topic_prefix, self.hostname),
        });
        let shutdown_json = serde_json::to_vec(&shutdown_payload).unwrap();
        client
            .publish(shutdown_topic, QoS::AtLeastOnce, true, shutdown_json)
            .await?;

        // 8.15. Monitored Service Control Switch Discovery configurations
        let monitored_services = crate::service_status::parse_monitored_services();
        for svc in &monitored_services {
            let svc_switch_topic = format!(
                "{}/switch/sysmqttd_{}_service_{}/config",
                self.config.mqtt_topic_prefix, self.hostname, svc
            );
            let svc_switch_payload = serde_json::json!({
                "name": format!("{} Service Control", svc),
                "state_topic": format!("{}/binary_sensor/sysmqttd_{}/service_{}/state", self.config.mqtt_topic_prefix, self.hostname, svc),
                "command_topic": format!("{}/switch/sysmqttd_{}_service_{}/set", self.config.mqtt_topic_prefix, self.hostname, svc),
                "unique_id": format!("sysmqttd_{}_service_{}_control", self.hostname, svc),
                "payload_on": "ON",
                "payload_off": "OFF",
                "availability_topic": format!("{}/sensor/sysmqttd_{}/availability", self.config.mqtt_topic_prefix, self.hostname),
                "device": device,
            });
            let svc_switch_json = serde_json::to_vec(&svc_switch_payload).unwrap();
            client
                .publish(svc_switch_topic, QoS::AtLeastOnce, true, svc_switch_json)
                .await?;
        }

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

        // 11. Telemetry Interval Discovery configuration (as a Home Assistant number entity)
        let interval_discovery_topic = format!(
            "{}/number/sysmqttd_{}_interval/config",
            self.config.mqtt_topic_prefix, self.hostname
        );
        let interval_payload = serde_json::json!({
            "name": "Telemetry Interval",
            "state_topic": format!("{}/sensor/sysmqttd_{}/interval/state", self.config.mqtt_topic_prefix, self.hostname),
            "command_topic": format!("{}/sensor/sysmqttd_{}/interval/set", self.config.mqtt_topic_prefix, self.hostname),
            "unique_id": format!("sysmqttd_{}_interval", self.hostname),
            "min": 1,
            "max": 86400,
            "step": 1,
            "unit_of_measurement": "s",
            "device": device,
            "availability_topic": format!("{}/sensor/sysmqttd_{}/availability", self.config.mqtt_topic_prefix, self.hostname),
        });
        let interval_discovery_json = serde_json::to_vec(&interval_payload).unwrap();
        client
            .publish(
                interval_discovery_topic,
                QoS::AtLeastOnce,
                true,
                interval_discovery_json,
            )
            .await?;

        println!("Published Home Assistant MQTT Discovery configs successfully.");
        Ok(())
    }

    /// Run the main daemon loop with support for a clean shutdown signal
    pub async fn run_with_shutdown(
        self,
        mut shutdown_rx: tokio::sync::oneshot::Receiver<()>,
    ) -> Result<(), String> {
        let mqttoptions = self
            .get_mqtt_options()
            .map_err(|e| format!("TLS/MQTT Option configuration error: {}", e))?;
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 100);

        let (interval_tx, interval_rx) = tokio::sync::watch::channel(60u64);

        // Spawn Telemetry Loop
        self.spawn_telemetry_loop(client.clone(), interval_rx.clone());
        // Spawn Service Status Loop
        self.spawn_service_status_loop(client.clone());
        // Spawn GPIO Inputs Polling Loop
        self.spawn_gpio_inputs_loop(client.clone());
        // Setup GPIO Outputs
        self.setup_gpio_outputs(client.clone());

        let mut backoff = crate::backoff::Backoff::new(
            Duration::from_secs(self.config.reconnect_initial_delay),
            Duration::from_secs(self.config.reconnect_max_delay),
        );
        let mut last_error_log_time: Option<tokio::time::Instant> = None;

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
                                    backoff.reset();
                                    // Publish Birth Message
                                    let availability_topic = format!(
                                        "{}/sensor/sysmqttd_{}/availability",
                                        self.config.mqtt_topic_prefix, self.hostname
                                    );
                                    let client_clone = client.clone();
                                    let self_clone = self.clone();
                                    tokio::spawn(async move {
                                        if let Err(e) = client_clone.publish(&availability_topic, QoS::AtLeastOnce, true, "online").await {
                                            eprintln!("Failed to publish online availability state: {}", e);
                                        }
                                        if let Err(e) = self_clone.publish_discovery(&client_clone).await {
                                            eprintln!("Failed to publish Home Assistant discovery configurations: {}", e);
                                        }
                                    });
                                    // Publish initial interval state
                                    let interval_state_topic = format!(
                                        "{}/sensor/sysmqttd_{}/interval/state",
                                        self.config.mqtt_topic_prefix, self.hostname
                                    );
                                    let init_interval = *interval_rx.borrow();
                                    let client_clone2 = client.clone();
                                    tokio::spawn(async move {
                                        if let Err(e) = client_clone2.publish(&interval_state_topic, QoS::AtLeastOnce, true, init_interval.to_string()).await {
                                            eprintln!("Failed to publish initial interval state: {}", e);
                                        }
                                    });
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
                                    // Subscribe to Monitored Service command topics
                                    let monitored_services = crate::service_status::parse_monitored_services();
                                    for svc in &monitored_services {
                                         let cmd_topic = format!(
                                             "{}/switch/sysmqttd_{}_service_{}/set",
                                             self.config.mqtt_topic_prefix, self.hostname, svc
                                         );
                                         if let Err(e) = client.subscribe(&cmd_topic, QoS::AtLeastOnce).await {
                                             eprintln!("Failed to subscribe to service command topic {}: {}", cmd_topic, e);
                                         }
                                    }
                                    // Subscribe to dynamic interval adjustment command topic
                                    let interval_cmd_topic = format!(
                                        "{}/sensor/sysmqttd_{}/interval/set",
                                        self.config.mqtt_topic_prefix, self.hostname
                                    );
                                    if let Err(e) = client.subscribe(&interval_cmd_topic, QoS::AtLeastOnce).await {
                                        eprintln!("Failed to subscribe to interval command topic {}: {}", interval_cmd_topic, e);
                                    }
                                }
                                Packet::Publish(publish) => {
                                    let prefix = &self.config.mqtt_topic_prefix;
                                    let hostname = &self.hostname;

                                    // Check if this publish is for our dynamic interval set topic
                                    let interval_set_topic = format!("{}/sensor/sysmqttd_{}/interval/set", prefix, hostname);
                                    if publish.topic == interval_set_topic {
                                        let payload_str = String::from_utf8_lossy(&publish.payload);
                                        let trimmed = payload_str.trim();
                                        match trimmed.parse::<u64>() {
                                            Ok(val) => {
                                                if (1..=86400).contains(&val) {
                                                    println!("Received valid dynamic polling interval adjustment: {}s", val);
                                                    if let Err(e) = interval_tx.send(val) {
                                                        eprintln!("Failed to update telemetry interval: {}", e);
                                                    } else {
                                                        // Publish confirmed state
                                                        let state_topic = format!(
                                                            "{}/sensor/sysmqttd_{}/interval/state",
                                                            prefix, hostname
                                                        );
                                                        let client_clone = client.clone();
                                                        tokio::spawn(async move {
                                                            if let Err(e) = client_clone.publish(&state_topic, QoS::AtLeastOnce, true, val.to_string()).await {
                                                                eprintln!("Failed to publish interval state confirmation: {}", e);
                                                            }
                                                        });
                                                    }
                                                } else {
                                                    eprintln!("Rejected interval adjustment command: value {} is out of bounds (1s to 86400s)", val);
                                                }
                                            }
                                            Err(_) => {
                                                eprintln!("Rejected invalid dynamic polling interval payload: '{}' (must be an integer)", trimmed);
                                            }
                                        }
                                    }

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

                                    // Check if this publish is for one of our monitored service command topics
                                    let monitored_services = crate::service_status::parse_monitored_services();
                                    for svc in &monitored_services {
                                        let cmd_topic = format!(
                                            "{}/switch/sysmqttd_{}_service_{}/set",
                                            prefix, hostname, svc
                                        );
                                        if publish.topic == cmd_topic {
                                            let payload_str = String::from_utf8_lossy(&publish.payload).trim().to_uppercase();
                                            let action = match payload_str.as_str() {
                                                "ON" => Some(crate::commands::RemoteAction::StartSystemdService(svc.clone())),
                                                "OFF" => Some(crate::commands::RemoteAction::StopSystemdService(svc.clone())),
                                                "RESTART" => Some(crate::commands::RemoteAction::RestartSystemdService(svc.clone())),
                                                _ => {
                                                    eprintln!("Unknown service command payload: {}", payload_str);
                                                    None
                                                }
                                            };

                                            if let Some(act) = action {
                                                if self.config.verbose {
                                                    println!("Executing whitelisted remote action: {:?}", act);
                                                }
                                                match act.execute() {
                                                    Ok(_) => {
                                                        if self.config.verbose {
                                                            println!("Successfully executed remote action: {:?}", act);
                                                        }
                                                        // Publish confirmed state back to binary sensor to update switch feedback
                                                        let state_topic = format!(
                                                            "{}/binary_sensor/sysmqttd_{}/service_{}/state",
                                                            prefix, hostname, svc
                                                        );
                                                        let confirmed_payload = match act {
                                                            crate::commands::RemoteAction::StopSystemdService(_) => "off",
                                                            _ => "on",
                                                        };
                                                        let client_clone = client.clone();
                                                        tokio::spawn(async move {
                                                            if let Err(e) = client_clone.publish(state_topic, QoS::AtLeastOnce, true, confirmed_payload).await {
                                                                eprintln!("Failed to publish service state confirmation: {}", e);
                                                            }
                                                        });
                                                    }
                                                    Err(e) => {
                                                        eprintln!("Failed to execute remote action {:?}: {}", act, e);
                                                    }
                                                }
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
                                                let controller = crate::gpio_outputs::GpioOutputController::with_base_path(
                                                    pin_config.pin,
                                                    pin_config.name.clone(),
                                                    self.gpio_base_path.clone(),
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
                            let retries = backoff.retries();
                            let delay = backoff.next_delay();
                            let now = tokio::time::Instant::now();
                            let elapsed_5_mins = last_error_log_time
                                .map(|t| now.duration_since(t) >= Duration::from_secs(300))
                                .unwrap_or(true);
                            let consecutive_failures = retries + 1;
                            let is_milestone = consecutive_failures == 1
                                || consecutive_failures == 5
                                || consecutive_failures.is_multiple_of(10);

                            if is_milestone || elapsed_5_mins {
                                eprintln!(
                                    "MQTT EventLoop Error (consecutive failure #{}): {}. Retrying in {:?}",
                                    consecutive_failures, e, delay
                                );
                                last_error_log_time = Some(now);
                            }
                            time::sleep(delay).await;
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

    /// Run ephemeral diagnostic healthcheck sequence
    pub async fn run_healthcheck(&self) -> Result<(), HealthcheckError> {
        // 1. Run local telemetry gather
        println!("Checking local telemetry gathering...");
        let mut collector = telemetry::TelemetryCollector::with_sysfs_root(self.sysfs_root.clone());
        collector.temperature_unit = self.config.temperature_unit.clone();
        collector.no_fan = self.config.no_fan;

        // Verify network interface is readable/accessible
        if let Err(e) = collector.read_interface_bytes(&self.config.net_interface) {
            return Err(HealthcheckError::TelemetryError(format!(
                "Failed to read interface '{}' metrics: {}",
                self.config.net_interface, e
            )));
        }

        // Run single-cycle gather
        let _state = collector.collect(&self.config.net_interface);
        println!("Telemetry gathered successfully.");

        // 2. Ephemeral broker connection check
        println!("Verifying MQTT broker connection...");
        let mut mqttoptions = self
            .get_mqtt_options()
            .map_err(HealthcheckError::ConfigError)?;
        // Set a low timeout / reconnection settings so we fail quickly if the broker is not reachable
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 5);

        let connection_result = tokio::time::timeout(Duration::from_secs(3), async {
            loop {
                match eventloop.poll().await {
                    Ok(Event::Incoming(Packet::ConnAck(connack))) => {
                        if connack.code != rumqttc::ConnectReturnCode::Success {
                            return Err(HealthcheckError::BrokerError(format!(
                                "MQTT broker connection refused: {:?}",
                                connack.code
                            )));
                        }
                        return Ok(connack);
                    }
                    Ok(_) => {} // Ignore other events during handshake
                    Err(e) => {
                        return Err(HealthcheckError::BrokerError(format!(
                            "Failed to poll MQTT connection: {}",
                            e
                        )));
                    }
                }
            }
        })
        .await;

        match connection_result {
            Ok(Ok(_connack)) => {
                println!("MQTT connection handshake verified successfully.");
                // Cleanly disconnect
                if let Err(e) = client.disconnect().await {
                    eprintln!("Warning: failed to cleanly disconnect MQTT client: {}", e);
                }
            }
            Ok(Err(e)) => return Err(e),
            Err(_) => {
                return Err(HealthcheckError::BrokerError(
                    "MQTT broker connection timed out after 3 seconds".to_string(),
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum HealthcheckError {
    ConfigError(String),
    TelemetryError(String),
    BrokerError(String),
}

impl HealthcheckError {
    pub fn exit_code(&self) -> i32 {
        match self {
            HealthcheckError::ConfigError(_) => 1,
            HealthcheckError::TelemetryError(_) => 2,
            HealthcheckError::BrokerError(_) => 3,
        }
    }
}

impl std::fmt::Display for HealthcheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthcheckError::ConfigError(msg) => write!(f, "Configuration Error: {}", msg),
            HealthcheckError::TelemetryError(msg) => write!(f, "Telemetry Error: {}", msg),
            HealthcheckError::BrokerError(msg) => write!(f, "Broker Connection Error: {}", msg),
        }
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
            temperature_unit: "F".to_string(),
            use_tls: false,
            ca_cert_path: None,
            reconnect_initial_delay: 2,
            reconnect_max_delay: 300,
            sd_alert_threshold: 95.0,
            telemetry_interval: 60,
            no_fan: false,
        };
        let daemon = Daemon::new(config, "pi-zero".to_string());

        let options = daemon.get_mqtt_options().unwrap();
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
            temperature_unit: "F".to_string(),
            use_tls: false,
            ca_cert_path: None,
            reconnect_initial_delay: 2,
            reconnect_max_delay: 300,
            sd_alert_threshold: 95.0,
            telemetry_interval: 60,
            no_fan: false,
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
            temperature_unit: "F".to_string(),
            use_tls: false,
            ca_cert_path: None,
            reconnect_initial_delay: 2,
            reconnect_max_delay: 300,
            sd_alert_threshold: 95.0,
            telemetry_interval: 60,
            no_fan: false,
        };
        let daemon = Daemon::new(config, "pi-zero".to_string());
        assert_eq!(daemon.config.gpio_outputs.len(), 1);
        assert_eq!(daemon.config.gpio_outputs[0].pin, 24);
        assert_eq!(daemon.config.gpio_outputs[0].name, "Mock Switch");
    }

    #[tokio::test]
    async fn test_run_healthcheck_telemetry_failure() {
        let config = Config {
            mqtt_host: "127.0.0.1".to_string(),
            mqtt_port: 1883,
            mqtt_user: None,
            mqtt_password: None,
            mqtt_topic_prefix: "ha_home".to_string(),
            net_interface: "non_existent_interface_abc123".to_string(),
            gpio_inputs: vec![],
            gpio_outputs: vec![],
            verbose: false,
            temperature_unit: "F".to_string(),
            use_tls: false,
            ca_cert_path: None,
            reconnect_initial_delay: 2,
            reconnect_max_delay: 300,
            sd_alert_threshold: 95.0,
            telemetry_interval: 60,
            no_fan: false,
        };
        let daemon = Daemon::new(config, "pi-zero".to_string());
        let res = daemon.run_healthcheck().await;
        assert!(res.is_err());
        match res.err().unwrap() {
            HealthcheckError::TelemetryError(msg) => {
                assert!(msg.contains("non_existent_interface_abc123"));
            }
            other => panic!("Expected TelemetryError, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_run_healthcheck_broker_failure() {
        let config = Config {
            mqtt_host: "127.0.0.1".to_string(),
            mqtt_port: 59999, // Unlikely to be a broker running here
            mqtt_user: None,
            mqtt_password: None,
            mqtt_topic_prefix: "ha_home".to_string(),
            net_interface: "lo".to_string(),
            gpio_inputs: vec![],
            gpio_outputs: vec![],
            verbose: false,
            temperature_unit: "F".to_string(),
            use_tls: false,
            ca_cert_path: None,
            reconnect_initial_delay: 2,
            reconnect_max_delay: 300,
            sd_alert_threshold: 95.0,
            telemetry_interval: 60,
            no_fan: false,
        };
        let daemon = Daemon::new(config, "pi-zero".to_string());
        let res = daemon.run_healthcheck().await;
        assert!(res.is_err());
        match res.err().unwrap() {
            HealthcheckError::BrokerError(_) => {}
            other => panic!("Expected BrokerError, got {:?}", other),
        }
    }

    #[test]
    fn test_daemon_get_tls_config_native() {
        let config = Config {
            mqtt_host: "10.0.0.5".to_string(),
            mqtt_port: 8883,
            mqtt_user: None,
            mqtt_password: None,
            mqtt_topic_prefix: "ha_home".to_string(),
            net_interface: "wlan0".to_string(),
            gpio_inputs: vec![],
            gpio_outputs: vec![],
            verbose: false,
            temperature_unit: "F".to_string(),
            use_tls: true,
            ca_cert_path: None,
            reconnect_initial_delay: 2,
            reconnect_max_delay: 300,
            sd_alert_threshold: 95.0,
            telemetry_interval: 60,
            no_fan: false,
        };
        let daemon = Daemon::new(config, "pi-zero".to_string());
        let tls_config = daemon.get_tls_config();
        assert!(tls_config.is_ok());
    }

    #[test]
    fn test_daemon_get_tls_config_invalid_ca() {
        let config = Config {
            mqtt_host: "10.0.0.5".to_string(),
            mqtt_port: 8883,
            mqtt_user: None,
            mqtt_password: None,
            mqtt_topic_prefix: "ha_home".to_string(),
            net_interface: "wlan0".to_string(),
            gpio_inputs: vec![],
            gpio_outputs: vec![],
            verbose: false,
            temperature_unit: "F".to_string(),
            use_tls: true,
            ca_cert_path: Some("invalid_path_to_ca_cert_123.pem".to_string()),
            reconnect_initial_delay: 2,
            reconnect_max_delay: 300,
            sd_alert_threshold: 95.0,
            telemetry_interval: 60,
            no_fan: false,
        };
        let daemon = Daemon::new(config, "pi-zero".to_string());
        let tls_config = daemon.get_tls_config();
        assert!(tls_config.is_err());
        assert!(tls_config
            .err()
            .unwrap()
            .contains("Failed to open CA certificate file"));
    }

    #[tokio::test]
    async fn test_spawn_telemetry_loop_dynamic_interval() {
        let config = Config {
            mqtt_host: "127.0.0.1".to_string(),
            mqtt_port: 1883,
            mqtt_user: None,
            mqtt_password: None,
            mqtt_topic_prefix: "ha_home".to_string(),
            net_interface: "lo".to_string(),
            gpio_inputs: vec![],
            gpio_outputs: vec![],
            verbose: false,
            temperature_unit: "F".to_string(),
            use_tls: false,
            ca_cert_path: None,
            reconnect_initial_delay: 2,
            reconnect_max_delay: 300,
            sd_alert_threshold: 95.0,
            telemetry_interval: 5,
            no_fan: false,
        };
        let daemon = Daemon::new(config, "pi-zero".to_string());

        let mqttoptions = daemon.get_mqtt_options().unwrap();
        let (client, _eventloop) = AsyncClient::new(mqttoptions, 10);
        let (interval_tx, interval_rx) = tokio::sync::watch::channel(5u64);

        daemon.spawn_telemetry_loop(client, interval_rx);

        // Initially, the interval should be 5 seconds
        assert_eq!(*interval_tx.borrow(), 5);

        // Update the interval to 10 seconds
        interval_tx.send(10).unwrap();
        assert_eq!(*interval_tx.borrow(), 10);
    }
}
