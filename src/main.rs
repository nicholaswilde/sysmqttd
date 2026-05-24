mod config;

use config::Config;
use rumqttc::{AsyncClient, MqttOptions, Event, Packet};
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

    // 4. Run event loop in a dedicated task or main loop
    println!("Connecting to MQTT broker...");
    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                match notification {
                    Event::Incoming(Packet::ConnAck(connack)) => {
                        println!("Successfully connected to MQTT broker! ConnAck: {:?}", connack);
                    }
                    Event::Incoming(incoming) => {
                        // Diagnostic log (optional, e.g. for sub/pub ack)
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

// A simple hostname module since we want to avoid extra crates where possible.
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
