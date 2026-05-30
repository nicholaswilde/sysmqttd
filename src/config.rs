use crate::gpio_inputs::{parse_gpio_inputs_env, GpioInputConfig};
use crate::gpio_outputs::{parse_gpio_outputs_env, GpioOutputConfig};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct CliOverrides {
    pub config_path: Option<String>,
    pub mqtt_host: Option<String>,
    pub mqtt_port: Option<u16>,
    pub mqtt_user: Option<String>,
    pub mqtt_password: Option<String>,
    pub mqtt_topic_prefix: Option<String>,
    pub net_interface: Option<String>,
    pub monitored_services: Option<String>,
    pub gpio_inputs: Option<String>,
    pub gpio_outputs: Option<String>,
    pub verbose: Option<bool>,
    pub temperature_unit: Option<String>,
    pub use_tls: Option<bool>,
    pub ca_cert_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Config {
    #[serde(alias = "host")]
    pub mqtt_host: String,
    #[serde(alias = "port", default = "default_mqtt_port")]
    pub mqtt_port: u16,
    #[serde(alias = "user", alias = "username")]
    pub mqtt_user: Option<String>,
    #[serde(alias = "password", alias = "pass")]
    pub mqtt_password: Option<String>,
    #[serde(alias = "prefix", default = "default_mqtt_prefix")]
    pub mqtt_topic_prefix: String,
    #[serde(alias = "interface", default = "default_net_interface")]
    pub net_interface: String,
    #[serde(default)]
    pub gpio_inputs: Vec<GpioInputConfig>,
    #[serde(default)]
    pub gpio_outputs: Vec<GpioOutputConfig>,
    #[serde(default)]
    pub verbose: bool,
    #[serde(
        alias = "temperature_unit",
        alias = "unit",
        alias = "temp_unit",
        default = "default_temperature_unit"
    )]
    pub temperature_unit: String,
    #[serde(alias = "use_tls", alias = "tls", default)]
    pub use_tls: bool,
    #[serde(alias = "ca_cert_path", alias = "ca_path")]
    pub ca_cert_path: Option<String>,
}

fn default_mqtt_port() -> u16 {
    1883
}

fn default_mqtt_prefix() -> String {
    "homeassistant".to_string()
}

fn default_net_interface() -> String {
    "wlan0".to_string()
}

fn default_temperature_unit() -> String {
    "F".to_string()
}

#[derive(serde::Deserialize, Default, Clone)]
pub struct FileConfig {
    #[serde(alias = "host")]
    pub mqtt_host: Option<String>,
    #[serde(alias = "port")]
    pub mqtt_port: Option<u16>,
    #[serde(alias = "user", alias = "username")]
    pub mqtt_user: Option<String>,
    #[serde(alias = "password", alias = "pass")]
    pub mqtt_password: Option<String>,
    #[serde(alias = "prefix")]
    pub mqtt_topic_prefix: Option<String>,
    #[serde(alias = "interface")]
    pub net_interface: Option<String>,
    #[serde(alias = "gpio_inputs")]
    pub gpio_inputs: Option<Vec<GpioInputConfig>>,
    #[serde(alias = "gpio_outputs")]
    pub gpio_outputs: Option<Vec<GpioOutputConfig>>,
    #[serde(alias = "verbose")]
    pub verbose: Option<bool>,
    #[serde(alias = "temperature_unit", alias = "unit", alias = "temp_unit")]
    pub temperature_unit: Option<String>,
    #[serde(alias = "use_tls", alias = "tls")]
    pub use_tls: Option<bool>,
    #[serde(alias = "ca_cert_path", alias = "ca_path")]
    pub ca_cert_path: Option<String>,
}

fn parse_file_content(path: &str, content: &str) -> Result<FileConfig, String> {
    let p = path.to_lowercase();
    if p.ends_with(".toml") {
        toml::from_str(content).map_err(|e| format!("Failed to parse TOML config: {}", e))
    } else if p.ends_with(".yaml") || p.ends_with(".yml") {
        serde_yaml::from_str(content).map_err(|e| format!("Failed to parse YAML config: {}", e))
    } else if p.ends_with(".json") {
        serde_json::from_str(content).map_err(|e| format!("Failed to parse JSON config: {}", e))
    } else {
        // Fallback: try auto-detecting
        if let Ok(cfg) = toml::from_str(content) {
            return Ok(cfg);
        }
        if let Ok(cfg) = serde_yaml::from_str(content) {
            return Ok(cfg);
        }
        if let Ok(cfg) = serde_json::from_str(content) {
            return Ok(cfg);
        }
        Err("Unknown configuration file extension and auto-detection failed".to_string())
    }
}

impl Config {
    /// Load configurations from fallback files and environment variables.
    /// Environment variables have higher precedence.
    pub fn load() -> Result<Self, String> {
        Self::load_with_overrides(CliOverrides::default())
    }

    /// Load configuration using a custom config file path or falling back to defaults.
    pub fn load_with_path(custom_path: Option<&str>) -> Result<Self, String> {
        Self::load_with_overrides(CliOverrides {
            config_path: custom_path.map(|s| s.to_string()),
            ..CliOverrides::default()
        })
    }

    /// Load configuration with CLI overrides.
    /// CLI overrides take the highest precedence.
    pub fn load_with_overrides(overrides: CliOverrides) -> Result<Self, String> {
        let mut file_config = FileConfig::default();

        if let Some(path) = &overrides.config_path {
            if Path::new(path).exists() {
                let content = fs::read_to_string(path)
                    .map_err(|e| format!("Failed to read custom config file '{}': {}", path, e))?;
                file_config = parse_file_content(path, &content)?;
            } else {
                return Err(format!(
                    "Custom configuration file '{}' does not exist",
                    path
                ));
            }
        } else {
            // Check default paths in order
            let default_paths = [
                "sysmqttd.toml",
                "sysmqttd.yaml",
                "sysmqttd.yml",
                "sysmqttd.json",
                "/etc/sysmqttd/sysmqttd.toml",
                "/etc/sysmqttd/sysmqttd.yaml",
                "/etc/sysmqttd/sysmqttd.yml",
                "/etc/sysmqttd/sysmqttd.json",
            ];
            for path in default_paths.iter() {
                if Path::new(path).exists() {
                    if let Ok(content) = fs::read_to_string(path) {
                        file_config = parse_file_content(path, &content)?;
                        break; // Stop at first found config file
                    }
                }
            }
        }

        // Environment variables prefixed with SYSMQTTD_ take highest precedence,
        // followed by legacy MQTT_ env vars, then file configs, then defaults.
        let mqtt_host = overrides
            .mqtt_host
            .or_else(|| env::var("SYSMQTTD_MQTT_HOST").ok())
            .or_else(|| env::var("MQTT_HOST").ok())
            .or(file_config.mqtt_host)
            .ok_or_else(|| "Missing required SYSMQTTD_MQTT_HOST configuration".to_string())?;

        let use_tls = overrides
            .use_tls
            .or_else(|| {
                env::var("SYSMQTTD_USE_TLS").ok().map(|v| {
                    let v_lower = v.to_lowercase();
                    v_lower == "true" || v_lower == "1" || v_lower == "yes"
                })
            })
            .or_else(|| {
                env::var("USE_TLS").ok().map(|v| {
                    let v_lower = v.to_lowercase();
                    v_lower == "true" || v_lower == "1" || v_lower == "yes"
                })
            })
            .or(file_config.use_tls)
            .unwrap_or(false);

        let ca_cert_path = overrides
            .ca_cert_path
            .or_else(|| env::var("SYSMQTTD_CA_CERT_PATH").ok())
            .or_else(|| env::var("CA_CERT_PATH").ok())
            .or(file_config.ca_cert_path);

        let mqtt_port = overrides
            .mqtt_port
            .or_else(|| {
                env::var("SYSMQTTD_MQTT_PORT")
                    .ok()
                    .and_then(|p| p.parse::<u16>().ok())
            })
            .or_else(|| {
                env::var("MQTT_PORT")
                    .ok()
                    .and_then(|p| p.parse::<u16>().ok())
            })
            .or(file_config.mqtt_port)
            .unwrap_or_else(|| {
                if use_tls {
                    8883
                } else {
                    1883
                }
            });

        let mqtt_user = overrides
            .mqtt_user
            .or_else(|| env::var("SYSMQTTD_MQTT_USER").ok())
            .or_else(|| env::var("MQTT_USER").ok())
            .or(file_config.mqtt_user);

        let mqtt_password = overrides
            .mqtt_password
            .or_else(|| env::var("SYSMQTTD_MQTT_PASSWORD").ok())
            .or_else(|| env::var("MQTT_PASSWORD").ok())
            .or(file_config.mqtt_password);

        let mqtt_topic_prefix = overrides
            .mqtt_topic_prefix
            .or_else(|| env::var("SYSMQTTD_MQTT_TOPIC_PREFIX").ok())
            .or_else(|| env::var("MQTT_TOPIC_PREFIX").ok())
            .or(file_config.mqtt_topic_prefix)
            .unwrap_or_else(|| "homeassistant".to_string());

        let net_interface = overrides
            .net_interface
            .or_else(|| env::var("SYSMQTTD_NET_INTERFACE").ok())
            .or_else(|| env::var("NET_INTERFACE").ok())
            .or(file_config.net_interface)
            .unwrap_or_else(|| "wlan0".to_string());

        if let Some(services) = overrides.monitored_services {
            env::set_var("MONITORED_SERVICES", services);
        }

        let mut gpio_inputs = file_config.gpio_inputs.unwrap_or_default();

        if let Ok(env_gpio) = env::var("SYSMQTTD_GPIO_INPUTS").or_else(|_| env::var("GPIO_INPUTS"))
        {
            gpio_inputs = parse_gpio_inputs_env(&env_gpio);
        }

        if let Some(cli_gpio) = &overrides.gpio_inputs {
            gpio_inputs = parse_gpio_inputs_env(cli_gpio);
        }

        let mut gpio_outputs = file_config.gpio_outputs.unwrap_or_default();

        if let Ok(env_gpio_out) =
            env::var("SYSMQTTD_GPIO_OUTPUTS").or_else(|_| env::var("GPIO_OUTPUTS"))
        {
            gpio_outputs = parse_gpio_outputs_env(&env_gpio_out);
        }

        if let Some(cli_gpio_out) = &overrides.gpio_outputs {
            gpio_outputs = parse_gpio_outputs_env(cli_gpio_out);
        }

        let verbose = overrides
            .verbose
            .or_else(|| {
                env::var("SYSMQTTD_VERBOSE").ok().map(|v| {
                    let v_lower = v.to_lowercase();
                    v_lower == "true" || v_lower == "1" || v_lower == "yes"
                })
            })
            .or(file_config.verbose)
            .unwrap_or(false);

        let temperature_unit = overrides
            .temperature_unit
            .or_else(|| env::var("SYSMQTTD_TEMPERATURE_UNIT").ok())
            .or_else(|| env::var("TEMPERATURE_UNIT").ok())
            .or(file_config.temperature_unit)
            .unwrap_or_else(|| "F".to_string())
            .trim()
            .to_uppercase();

        if temperature_unit != "C" && temperature_unit != "F" {
            return Err(format!(
                "Invalid temperature unit '{}'. Must be 'C' or 'F' (case-insensitive)",
                temperature_unit
            ));
        }

        Ok(Config {
            mqtt_host,
            mqtt_port,
            mqtt_user,
            mqtt_password,
            mqtt_topic_prefix,
            net_interface,
            gpio_inputs,
            gpio_outputs,
            verbose,
            temperature_unit,
            use_tls,
            ca_cert_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn clean_env() {
        env::remove_var("SYSMQTTD_MQTT_HOST");
        env::remove_var("SYSMQTTD_MQTT_PORT");
        env::remove_var("SYSMQTTD_MQTT_USER");
        env::remove_var("SYSMQTTD_MQTT_PASSWORD");
        env::remove_var("SYSMQTTD_MQTT_TOPIC_PREFIX");
        env::remove_var("SYSMQTTD_NET_INTERFACE");
        env::remove_var("MONITORED_SERVICES");
        env::remove_var("SYSMQTTD_TEMPERATURE_UNIT");
        env::remove_var("SYSMQTTD_USE_TLS");
        env::remove_var("SYSMQTTD_CA_CERT_PATH");

        env::remove_var("MQTT_HOST");
        env::remove_var("MQTT_PORT");
        env::remove_var("MQTT_USER");
        env::remove_var("MQTT_PASSWORD");
        env::remove_var("MQTT_TOPIC_PREFIX");
        env::remove_var("NET_INTERFACE");
        env::remove_var("TEMPERATURE_UNIT");
        env::remove_var("USE_TLS");
        env::remove_var("CA_CERT_PATH");

        let _ = fs::remove_file("sysmqttd.toml");
        let _ = fs::remove_file("sysmqttd.yaml");
        let _ = fs::remove_file("sysmqttd.yml");
        let _ = fs::remove_file("sysmqttd.json");
        let _ = fs::remove_file("custom.toml");
        let _ = fs::remove_file("custom.yaml");
        let _ = fs::remove_file("custom.json");
    }

    #[test]
    fn test_config_suite_expanded() {
        clean_env();

        // 1. Test missing host error
        let result = Config::load();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Missing required SYSMQTTD_MQTT_HOST configuration"
        );

        // 2. Test legacy ENV loading
        env::set_var("MQTT_HOST", "127.0.0.1");
        env::set_var("MQTT_PORT", "1883");
        env::set_var("MQTT_USER", "legacy_user");
        env::set_var("MQTT_PASSWORD", "legacy_password");
        env::set_var("MQTT_TOPIC_PREFIX", "legacy_prefix");
        env::set_var("NET_INTERFACE", "eth0");

        let config = Config::load().unwrap();
        assert_eq!(config.mqtt_host, "127.0.0.1");
        assert_eq!(config.mqtt_port, 1883);
        assert_eq!(config.mqtt_user, Some("legacy_user".to_string()));
        assert_eq!(config.mqtt_password, Some("legacy_password".to_string()));
        assert_eq!(config.mqtt_topic_prefix, "legacy_prefix");
        assert_eq!(config.net_interface, "eth0");

        // 3. Test SYSMQTTD_ prefixed ENV overriding legacy ENV
        env::set_var("SYSMQTTD_MQTT_HOST", "10.0.0.2");
        env::set_var("SYSMQTTD_MQTT_PORT", "9999");
        let config_prefixed = Config::load().unwrap();
        assert_eq!(config_prefixed.mqtt_host, "10.0.0.2");
        assert_eq!(config_prefixed.mqtt_port, 9999);
        assert_eq!(config_prefixed.mqtt_user, Some("legacy_user".to_string())); // Prefixed not set for user, so falls back to legacy

        clean_env();

        // 4. Test TOML file loading
        {
            let mut file = File::create("sysmqttd.toml").unwrap();
            writeln!(
                file,
                r#"
                host = "192.168.1.100"
                port = 8883
                user = "toml_user"
                password = "toml_password"
                prefix = "toml_prefix"
                interface = "eth1"
                "#
            )
            .unwrap();
        }
        let config_toml = Config::load().unwrap();
        assert_eq!(config_toml.mqtt_host, "192.168.1.100");
        assert_eq!(config_toml.mqtt_port, 8883);
        assert_eq!(config_toml.mqtt_user, Some("toml_user".to_string()));
        assert_eq!(config_toml.mqtt_password, Some("toml_password".to_string()));
        assert_eq!(config_toml.mqtt_topic_prefix, "toml_prefix");
        assert_eq!(config_toml.net_interface, "eth1");

        clean_env();

        // 5. Test YAML file loading
        {
            let mut file = File::create("sysmqttd.yaml").unwrap();
            writeln!(
                file,
                r#"
                host: "192.168.1.200"
                port: 7777
                user: "yaml_user"
                password: "yaml_password"
                prefix: "yaml_prefix"
                interface: "eth2"
                "#
            )
            .unwrap();
        }
        let config_yaml = Config::load().unwrap();
        assert_eq!(config_yaml.mqtt_host, "192.168.1.200");
        assert_eq!(config_yaml.mqtt_port, 7777);
        assert_eq!(config_yaml.mqtt_user, Some("yaml_user".to_string()));
        assert_eq!(config_yaml.mqtt_password, Some("yaml_password".to_string()));
        assert_eq!(config_yaml.mqtt_topic_prefix, "yaml_prefix");
        assert_eq!(config_yaml.net_interface, "eth2");

        clean_env();

        // 6. Test JSON file loading
        {
            let mut file = File::create("sysmqttd.json").unwrap();
            writeln!(
                file,
                r#"{{
                "host": "192.168.1.250",
                "port": 5555,
                "user": "json_user",
                "password": "json_password",
                "prefix": "json_prefix",
                "interface": "eth3"
                }}"#
            )
            .unwrap();
        }
        let config_json = Config::load().unwrap();
        assert_eq!(config_json.mqtt_host, "192.168.1.250");
        assert_eq!(config_json.mqtt_port, 5555);
        assert_eq!(config_json.mqtt_user, Some("json_user".to_string()));
        assert_eq!(config_json.mqtt_password, Some("json_password".to_string()));
        assert_eq!(config_json.mqtt_topic_prefix, "json_prefix");
        assert_eq!(config_json.net_interface, "eth3");

        // 7. Test custom path CLI overriding
        clean_env();
        {
            let mut file = File::create("custom.json").unwrap();
            writeln!(
                file,
                r#"{{
                "host": "10.10.10.10",
                "port": 1234
                }}"#
            )
            .unwrap();
        }
        let config_custom = Config::load_with_path(Some("custom.json")).unwrap();
        assert_eq!(config_custom.mqtt_host, "10.10.10.10");
        assert_eq!(config_custom.mqtt_port, 1234);

        // 8. Test parameter CLI overrides
        clean_env();
        let overrides = CliOverrides {
            config_path: None,
            mqtt_host: Some("10.20.30.40".to_string()),
            mqtt_port: Some(8888),
            mqtt_user: Some("cli_user".to_string()),
            mqtt_password: Some("cli_pass".to_string()),
            mqtt_topic_prefix: Some("cli_prefix".to_string()),
            net_interface: Some("cli_eth".to_string()),
            monitored_services: Some("cli_svc1,cli_svc2".to_string()),
            gpio_inputs: None,
            gpio_outputs: None,
            verbose: None,
            temperature_unit: None,
            ..CliOverrides::default()
        };
        let config_overrides = Config::load_with_overrides(overrides).unwrap();
        assert_eq!(config_overrides.mqtt_host, "10.20.30.40");
        assert_eq!(config_overrides.mqtt_port, 8888);
        assert_eq!(config_overrides.mqtt_user, Some("cli_user".to_string()));
        assert_eq!(config_overrides.mqtt_password, Some("cli_pass".to_string()));
        assert_eq!(config_overrides.mqtt_topic_prefix, "cli_prefix");
        assert_eq!(config_overrides.net_interface, "cli_eth");
        assert_eq!(env::var("MONITORED_SERVICES").unwrap(), "cli_svc1,cli_svc2");

        clean_env();
    }

    #[test]
    fn test_verbose_config_overrides() {
        // 1. Default verbose is false
        clean_env();
        let overrides_default = CliOverrides {
            config_path: None,
            mqtt_host: Some("127.0.0.1".to_string()),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_default).unwrap();
        assert!(!cfg.verbose);

        // 2. CLI override verbose
        clean_env();
        let overrides_cli = CliOverrides {
            config_path: None,
            mqtt_host: Some("127.0.0.1".to_string()),
            verbose: Some(true),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_cli).unwrap();
        assert!(cfg.verbose);

        // 3. Env override verbose (SYSMQTTD_VERBOSE=true)
        clean_env();
        env::set_var("SYSMQTTD_VERBOSE", "true");
        let overrides_env = CliOverrides {
            config_path: None,
            mqtt_host: Some("127.0.0.1".to_string()),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_env).unwrap();
        assert!(cfg.verbose);

        // 4. Env override verbose (SYSMQTTD_VERBOSE=1)
        clean_env();
        env::set_var("SYSMQTTD_VERBOSE", "1");
        let overrides_env2 = CliOverrides {
            config_path: None,
            mqtt_host: Some("127.0.0.1".to_string()),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_env2).unwrap();
        assert!(cfg.verbose);

        clean_env();
        env::remove_var("SYSMQTTD_VERBOSE");
    }

    #[test]
    fn test_temperature_unit_config() {
        // 1. Default temperature unit is F
        clean_env();
        let overrides_default = CliOverrides {
            config_path: None,
            mqtt_host: Some("127.0.0.1".to_string()),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_default).unwrap();
        assert_eq!(cfg.temperature_unit, "F");

        // 2. TOML file parses temperature_unit = "C" (case-insensitive and alias)
        clean_env();
        {
            let mut file = File::create("sysmqttd.toml").unwrap();
            writeln!(
                file,
                r#"
                host = "127.0.0.1"
                unit = "c"
                "#
            )
            .unwrap();
        }
        let cfg = Config::load().unwrap();
        assert_eq!(cfg.temperature_unit, "C");

        // 3. CLI override sets it to "C"
        clean_env();
        let overrides_cli = CliOverrides {
            config_path: None,
            mqtt_host: Some("127.0.0.1".to_string()),
            temperature_unit: Some("c".to_string()),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_cli).unwrap();
        assert_eq!(cfg.temperature_unit, "C");

        // 4. Env var sets it to "C" (SYSMQTTD_TEMPERATURE_UNIT)
        clean_env();
        env::set_var("SYSMQTTD_TEMPERATURE_UNIT", "c");
        let overrides_env = CliOverrides {
            config_path: None,
            mqtt_host: Some("127.0.0.1".to_string()),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_env).unwrap();
        assert_eq!(cfg.temperature_unit, "C");

        // 5. Env var sets it to "C" (legacy TEMPERATURE_UNIT)
        clean_env();
        env::set_var("TEMPERATURE_UNIT", "C");
        let overrides_env2 = CliOverrides {
            config_path: None,
            mqtt_host: Some("127.0.0.1".to_string()),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_env2).unwrap();
        assert_eq!(cfg.temperature_unit, "C");

        // 6. Invalid values return error
        clean_env();
        let overrides_invalid = CliOverrides {
            config_path: None,
            mqtt_host: Some("127.0.0.1".to_string()),
            temperature_unit: Some("X".to_string()),
            ..CliOverrides::default()
        };
        let cfg_err = Config::load_with_overrides(overrides_invalid);
        assert!(cfg_err.is_err());
        assert!(cfg_err.err().unwrap().contains("Invalid temperature unit"));

        clean_env();
    }

    #[test]
    fn test_tls_config() {
        clean_env();

        // 1. Defaults: TLS is inactive, port is 1883
        let overrides_default = CliOverrides {
            mqtt_host: Some("127.0.0.1".to_string()),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_default).unwrap();
        assert_eq!(cfg.use_tls, false);
        assert_eq!(cfg.mqtt_port, 1883);
        assert_eq!(cfg.ca_cert_path, None);

        // 2. CLI override use_tls -> active, defaults port to 8883
        clean_env();
        let overrides_tls = CliOverrides {
            mqtt_host: Some("127.0.0.1".to_string()),
            use_tls: Some(true),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_tls).unwrap();
        assert_eq!(cfg.use_tls, true);
        assert_eq!(cfg.mqtt_port, 8883);

        // 3. Env var SYSMQTTD_USE_TLS -> active
        clean_env();
        env::set_var("SYSMQTTD_USE_TLS", "yes");
        let overrides_env = CliOverrides {
            mqtt_host: Some("127.0.0.1".to_string()),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_env).unwrap();
        assert_eq!(cfg.use_tls, true);
        assert_eq!(cfg.mqtt_port, 8883);

        // 4. Env var SYSMQTTD_CA_CERT_PATH
        clean_env();
        env::set_var("SYSMQTTD_CA_CERT_PATH", "/etc/ssl/certs/ca.pem");
        let overrides_env2 = CliOverrides {
            mqtt_host: Some("127.0.0.1".to_string()),
            ..CliOverrides::default()
        };
        let cfg = Config::load_with_overrides(overrides_env2).unwrap();
        assert_eq!(cfg.ca_cert_path, Some("/etc/ssl/certs/ca.pem".to_string()));

        // 5. TOML configuration file sets use_tls and ca_cert_path
        clean_env();
        {
            let mut file = File::create("sysmqttd.toml").unwrap();
            writeln!(
                file,
                r#"
                host = "127.0.0.1"
                use_tls = true
                ca_cert_path = "/custom/ca.crt"
                "#
            )
            .unwrap();
        }
        let cfg = Config::load().unwrap();
        assert_eq!(cfg.use_tls, true);
        assert_eq!(cfg.mqtt_port, 8883);
        assert_eq!(cfg.ca_cert_path, Some("/custom/ca.crt".to_string()));

        clean_env();
    }
}
