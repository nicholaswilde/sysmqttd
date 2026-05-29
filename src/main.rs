use sysmqttd::config::Config;
use sysmqttd::daemon::Daemon;
mod cli;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Parse CLI arguments before any other work
    let cli_overrides = match cli::parse_arguments(std::env::args().collect()) {
        Ok(cli::CliAction::PrintHelp) => {
            println!("{}", cli::usage());
            std::process::exit(0);
        }
        Ok(cli::CliAction::PrintVersion) => {
            println!("sysmqttd v{}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        Ok(cli::CliAction::Boot {
            config_path,
            mqtt_host,
            mqtt_port,
            mqtt_user,
            mqtt_password,
            mqtt_topic_prefix,
            net_interface,
            monitored_services,
            gpio_inputs,
            gpio_outputs,
            verbose,
            temperature_unit,
        }) => sysmqttd::config::CliOverrides {
            config_path,
            mqtt_host,
            mqtt_port,
            mqtt_user,
            mqtt_password,
            mqtt_topic_prefix,
            net_interface,
            monitored_services,
            gpio_inputs,
            gpio_outputs,
            verbose,
            temperature_unit,
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    println!("Starting sysmqttd system monitoring daemon...");

    // 1. Load configuration
    let config = match Config::load_with_overrides(cli_overrides) {
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
