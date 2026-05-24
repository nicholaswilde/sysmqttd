// CLI argument parsing for sysmqttd

/// Represents the action requested via command line.
#[derive(Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum CliAction {
    /// Print help/usage information and exit.
    PrintHelp,
    /// Print the version of the binary and exit.
    PrintVersion,
    /// Proceed with normal daemon boot.
    Boot {
        config_path: Option<String>,
        mqtt_host: Option<String>,
        mqtt_port: Option<u16>,
        mqtt_user: Option<String>,
        mqtt_password: Option<String>,
        mqtt_topic_prefix: Option<String>,
        net_interface: Option<String>,
        monitored_services: Option<String>,
        gpio_inputs: Option<String>,
        gpio_outputs: Option<String>,
        verbose: Option<bool>,
    },
}

/// Parse command line arguments.
///
/// Returns a `CliAction` on success or an error string describing the problem.
pub fn parse_arguments(args: Vec<String>) -> Result<CliAction, String> {
    let mut config_path = None;
    let mut mqtt_host = None;
    let mut mqtt_port = None;
    let mut mqtt_user = None;
    let mut mqtt_password = None;
    let mut mqtt_topic_prefix = None;
    let mut net_interface = None;
    let mut monitored_services = None;
    let mut gpio_inputs = None;
    let mut gpio_outputs = None;
    let mut verbose = None;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-h" | "--help" => return Ok(CliAction::PrintHelp),
            "-v" | "--version" => return Ok(CliAction::PrintVersion),
            "-c" | "--config" => {
                if i + 1 < args.len() {
                    config_path = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Missing path after configuration flag".to_string());
                }
            }
            "-H" | "--host" => {
                if i + 1 < args.len() {
                    mqtt_host = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Missing host after host flag".to_string());
                }
            }
            "-P" | "--port" => {
                if i + 1 < args.len() {
                    let val = &args[i + 1];
                    let port = val
                        .parse::<u16>()
                        .map_err(|e| format!("Invalid port value '{}': {}", val, e))?;
                    mqtt_port = Some(port);
                    i += 2;
                } else {
                    return Err("Missing port after port flag".to_string());
                }
            }
            "-u" | "--user" | "--username" => {
                if i + 1 < args.len() {
                    mqtt_user = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Missing username after user flag".to_string());
                }
            }
            "-w" | "--password" | "--pass" => {
                if i + 1 < args.len() {
                    mqtt_password = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Missing password after password flag".to_string());
                }
            }
            "-p" | "--prefix" => {
                if i + 1 < args.len() {
                    mqtt_topic_prefix = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Missing prefix after prefix flag".to_string());
                }
            }
            "-i" | "--interface" => {
                if i + 1 < args.len() {
                    net_interface = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Missing interface after interface flag".to_string());
                }
            }
            "-s" | "--services" | "--monitored-services" => {
                if i + 1 < args.len() {
                    monitored_services = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Missing services after services flag".to_string());
                }
            }
            "-g" | "--gpio" | "--gpio-inputs" => {
                if i + 1 < args.len() {
                    gpio_inputs = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Missing GPIO inputs after GPIO flag".to_string());
                }
            }
            "-o" | "--gpio-outputs" => {
                if i + 1 < args.len() {
                    gpio_outputs = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err("Missing GPIO outputs after outputs flag".to_string());
                }
            }
            "--verbose" => {
                verbose = Some(true);
                i += 1;
            }
            unknown => {
                return Err(format!("Unknown argument '{}'", unknown));
            }
        }
    }
    Ok(CliAction::Boot {
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
    })
}

/// Returns a short usage string printed for the `--help` flag.
pub fn usage() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        "sysmqttd {ver}\n\n\
Usage: sysmqttd [OPTIONS]\n\
Options:\n\
    -h, --help               Print this help message and exit\n\
    -v, --version            Print version information and exit\n\
    -c, --config <path>      Specify custom path to configuration file (TOML, YAML, JSON)\n\
    -H, --host <host>        MQTT broker host (e.g., localhost)\n\
    -P, --port <port>        MQTT broker port (default 1883)\n\
    -u, --user <username>    MQTT broker username\n\
    -w, --password <pass>    MQTT broker password\n\
    -p, --prefix <prefix>    Home Assistant discovery topic prefix (default homeassistant)\n\
    -i, --interface <if>     Network interface card (default wlan0)\n\
    -s, --services <list>    Comma-separated whitelist of systemd services to monitor\n\
    -g, --gpio <list>        Comma-separated whitelist of GPIO input pins\n\
    -o, --gpio-outputs <list> Comma-separated whitelist of GPIO output pins\n\
        --verbose            Enable verbose logging (payloads and packets detail)\n\n\
The daemon connects to an MQTT broker as configured via arguments, environment variables or a configuration file.\n",
        ver = version
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_args_boots() {
        let args = vec!["sysmqttd".to_string()];
        assert_eq!(
            parse_arguments(args).unwrap(),
            CliAction::Boot {
                config_path: None,
                mqtt_host: None,
                mqtt_port: None,
                mqtt_user: None,
                mqtt_password: None,
                mqtt_topic_prefix: None,
                net_interface: None,
                monitored_services: None,
                gpio_inputs: None,
                gpio_outputs: None,
                verbose: None,
            }
        );
    }

    #[test]
    fn test_help_flag() {
        let args = vec!["sysmqttd".to_string(), "-h".to_string()];
        assert_eq!(parse_arguments(args).unwrap(), CliAction::PrintHelp);
    }

    #[test]
    fn test_version_flag() {
        let args = vec!["sysmqttd".to_string(), "--version".to_string()];
        assert_eq!(parse_arguments(args).unwrap(), CliAction::PrintVersion);
    }

    #[test]
    fn test_config_flag_valid() {
        let args = vec![
            "sysmqttd".to_string(),
            "-c".to_string(),
            "custom_cfg.json".to_string(),
        ];
        assert_eq!(
            parse_arguments(args).unwrap(),
            CliAction::Boot {
                config_path: Some("custom_cfg.json".to_string()),
                mqtt_host: None,
                mqtt_port: None,
                mqtt_user: None,
                mqtt_password: None,
                mqtt_topic_prefix: None,
                net_interface: None,
                monitored_services: None,
                gpio_inputs: None,
                gpio_outputs: None,
                verbose: None,
            }
        );
    }

    #[test]
    fn test_all_parameter_flags_valid() {
        let args = vec![
            "sysmqttd".to_string(),
            "-H".to_string(),
            "10.0.0.5".to_string(),
            "-P".to_string(),
            "1884".to_string(),
            "-u".to_string(),
            "username".to_string(),
            "-w".to_string(),
            "password".to_string(),
            "-p".to_string(),
            "prefix".to_string(),
            "-i".to_string(),
            "eth0".to_string(),
            "-s".to_string(),
            "docker,nginx".to_string(),
        ];
        assert_eq!(
            parse_arguments(args).unwrap(),
            CliAction::Boot {
                config_path: None,
                mqtt_host: Some("10.0.0.5".to_string()),
                mqtt_port: Some(1884),
                mqtt_user: Some("username".to_string()),
                mqtt_password: Some("password".to_string()),
                mqtt_topic_prefix: Some("prefix".to_string()),
                net_interface: Some("eth0".to_string()),
                monitored_services: Some("docker,nginx".to_string()),
                gpio_inputs: None,
                gpio_outputs: None,
                verbose: None,
            }
        );
    }

    #[test]
    fn test_config_flag_missing_val() {
        let args = vec!["sysmqttd".to_string(), "--config".to_string()];
        assert!(parse_arguments(args).is_err());
    }

    #[test]
    fn test_port_invalid() {
        let args = vec!["sysmqttd".to_string(), "-P".to_string(), "abc".to_string()];
        assert!(parse_arguments(args).is_err());
    }

    #[test]
    fn test_unknown_flag() {
        let args = vec!["sysmqttd".to_string(), "--foo".to_string()];
        assert!(parse_arguments(args).is_err());
    }

    #[test]
    fn test_gpio_outputs_flag_valid() {
        let args = vec![
            "sysmqttd".to_string(),
            "-o".to_string(),
            "24:Relay 1,25:LED Indicator".to_string(),
        ];
        assert_eq!(
            parse_arguments(args).unwrap(),
            CliAction::Boot {
                config_path: None,
                mqtt_host: None,
                mqtt_port: None,
                mqtt_user: None,
                mqtt_password: None,
                mqtt_topic_prefix: None,
                net_interface: None,
                monitored_services: None,
                gpio_inputs: None,
                gpio_outputs: Some("24:Relay 1,25:LED Indicator".to_string()),
                verbose: None,
            }
        );
    }

    #[test]
    fn test_gpio_outputs_flag_missing_val() {
        let args = vec!["sysmqttd".to_string(), "-o".to_string()];
        assert!(parse_arguments(args).is_err());
    }

    #[test]
    fn test_verbose_flag_valid() {
        let args = vec!["sysmqttd".to_string(), "--verbose".to_string()];
        assert_eq!(
            parse_arguments(args).unwrap(),
            CliAction::Boot {
                config_path: None,
                mqtt_host: None,
                mqtt_port: None,
                mqtt_user: None,
                mqtt_password: None,
                mqtt_topic_prefix: None,
                net_interface: None,
                monitored_services: None,
                gpio_inputs: None,
                gpio_outputs: None,
                verbose: Some(true),
            }
        );
    }
}
