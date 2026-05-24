mod config;
mod daemon;
mod discovery;
mod telemetry;

use config::Config;
use daemon::Daemon;

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

    // 2. Set up host parameters
    let hostname = hostname::get_hostname().unwrap_or_else(|| "unknown-host".to_string());

    // 3. Instantiate and run Daemon
    let daemon = Daemon::new(config, hostname);
    if let Err(e) = daemon.run().await {
        eprintln!("Daemon execution failure: {}", e);
        std::process::exit(1);
    }
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
