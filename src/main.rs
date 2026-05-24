mod config;
mod discovery;

use config::Config;
use rumqttc::{AsyncClient, MqttOptions, Event, Packet, QoS};
use std::time::Duration;
use tokio::time;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Starting sysmqttd system monitoring daemon...");

    // 1. Load configuration
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration Error: {}", e);
            std::process::exit(1);
        }
    };

    println!("Configuration loaded successfully.");
    println!("MQTT Broker: {}:{}", config.mqtt_host, config.mqtt_port);
    println!("Topic Prefix: {}", config.mqtt_topic_prefix);

    // 2. Set up MQTT options
    let hostname = hostname::get_hostname().unwrap_or_else(|| "unknown-host".to_string());
    let client_id = format!("sysmqttd_{}", hostname);
    let mut mqttoptions = MqttOptions::new(client_id, &config.mqtt_host, config.mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    
    if let (Some(user), Some(pass)) = (&config.mqtt_user, &config.mqtt_password) {
        mqttoptions.set_credentials(user, pass);
    }

    // 3. Initialize client and event loop
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // 4. Run event loop
    println!("Connecting to MQTT broker...");
    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                match notification {
                    Event::Incoming(Packet::ConnAck(connack)) => {
                        println!("Successfully connected to MQTT broker! ConnAck: {:?}", connack);
                        
                        // Publish discovery payloads on successful connection (or reconnect)
                        if let Err(e) = publish_discovery_payloads(&client, &config.mqtt_topic_prefix, &hostname).await {
                            eprintln!("Failed to publish Home Assistant discovery configurations: {}", e);
                        }
                    }
                    Event::Incoming(incoming) => {
                        println!("Incoming packet: {:?}", incoming);
                    }
                    Event::Outgoing(outgoing) => {
                        println!("Outgoing packet: {:?}", outgoing);
                    }
                }
            }
            Err(e) => {
                eprintln!("MQTT EventLoop Error: {}. Retrying in 5 seconds...", e);
                time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

/// Publish Home Assistant Auto-Discovery Configuration payloads
async fn publish_discovery_payloads(client: &AsyncClient, prefix: &str, hostname: &str) -> Result<(), rumqttc::ClientError> {
    let device = discovery::DeviceInfo {
        identifiers: vec![format!("sysmqttd_{}", hostname)],
        name: format!("sysmqttd {}", hostname),
        model: "Raspberry Pi Zero W Monitor".to_string(),
        manufacturer: "sysmqttd".to_string(),
    };

    // 1. CPU Temperature Discovery configuration
    let cpu_payload = discovery::DiscoveryPayload::new_cpu_temp(prefix, hostname, device.clone());
    let cpu_topic = format!("{}/sensor/sysmqttd_{}_cpu_temp/config", prefix, hostname);
    let cpu_json = serde_json::to_vec(&cpu_payload).unwrap();
    client.publish(cpu_topic, QoS::AtLeastOnce, true, cpu_json).await?;

    // 2. RAM Usage Discovery configuration
    let ram_payload = discovery::DiscoveryPayload::new_ram_usage(prefix, hostname, device.clone());
    let ram_topic = format!("{}/sensor/sysmqttd_{}_ram_usage/config", prefix, hostname);
    let ram_json = serde_json::to_vec(&ram_payload).unwrap();
    client.publish(ram_topic, QoS::AtLeastOnce, true, ram_json).await?;

    // 3. Disk Usage Discovery configuration
    let disk_payload = discovery::DiscoveryPayload::new_disk_usage(prefix, hostname, device);
    let disk_topic = format!("{}/sensor/sysmqttd_{}_disk_usage/config", prefix, hostname);
    let disk_json = serde_json::to_vec(&disk_payload).unwrap();
    client.publish(disk_topic, QoS::AtLeastOnce, true, disk_json).await?;

    println!("Published Home Assistant MQTT Discovery configs successfully.");
    Ok(())
}

// A simple hostname module to get host name
mod hostname {
    use std::fs;
    pub fn get_hostname() -> Option<String> {
        // Try reading /proc/sys/kernel/hostname
        if let Ok(name) = fs::read_to_string("/proc/sys/kernel/hostname") {
            let trimmed = name.trim().to_string();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
        // Fallback to env var HOSTNAME
        std::env::var("HOSTNAME").ok()
    }
}
