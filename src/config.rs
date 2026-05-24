use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub mqtt_host: String,
    pub mqtt_port: u16,
    pub mqtt_user: Option<String>,
    pub mqtt_password: Option<String>,
    pub mqtt_topic_prefix: String,
}

impl Config {
    /// Load configurations from fallback files and environment variables.
    /// Environment variables have higher precedence.
    pub fn load() -> Result<Self, String> {
        let mut file_host = None;
        let mut file_port = None;
        let mut file_user = None;
        let mut file_password = None;
        let mut file_prefix = None;

        // Try reading local sysmqttd.toml first, then /etc/sysmqttd/sysmqttd.toml
        let paths = ["sysmqttd.toml", "/etc/sysmqttd/sysmqttd.toml"];
        for path in paths.iter() {
            if Path::new(path).exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.is_empty() || trimmed.starts_with('#') {
                            continue;
                        }
                        if let Some((key, val)) = trimmed.split_once('=') {
                            let k = key.trim().to_lowercase();
                            let v = val
                                .trim()
                                .trim_matches('"')
                                .trim_matches('\'')
                                .trim()
                                .to_string();
                            if v.is_empty() {
                                continue;
                            }
                            match k.as_str() {
                                "mqtt_host" | "host" => file_host = Some(v),
                                "mqtt_port" | "port" => {
                                    if let Ok(parsed_port) = v.parse::<u16>() {
                                        file_port = Some(parsed_port);
                                    }
                                }
                                "mqtt_user" | "user" | "username" => file_user = Some(v),
                                "mqtt_password" | "password" | "pass" => file_password = Some(v),
                                "mqtt_topic_prefix" | "prefix" => file_prefix = Some(v),
                                _ => {}
                            }
                        }
                    }
                }
                break; // Stop at first found config file
            }
        }

        // Environment variables override file configs
        let mqtt_host = env::var("MQTT_HOST")
            .ok()
            .or(file_host)
            .ok_or_else(|| "Missing required MQTT_HOST configuration".to_string())?;

        let mqtt_port = env::var("MQTT_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .or(file_port)
            .unwrap_or(1883);

        let mqtt_user = env::var("MQTT_USER").ok().or(file_user);
        let mqtt_password = env::var("MQTT_PASSWORD").ok().or(file_password);

        let mqtt_topic_prefix = env::var("MQTT_TOPIC_PREFIX")
            .ok()
            .or(file_prefix)
            .unwrap_or_else(|| "homeassistant".to_string());

        Ok(Config {
            mqtt_host,
            mqtt_port,
            mqtt_user,
            mqtt_password,
            mqtt_topic_prefix,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_config_suite() {
        // 1. Test missing host error (with clean env and no TOML)
        env::remove_var("MQTT_HOST");
        env::remove_var("MQTT_PORT");
        env::remove_var("MQTT_USER");
        env::remove_var("MQTT_PASSWORD");
        env::remove_var("MQTT_TOPIC_PREFIX");
        let _ = fs::remove_file("sysmqttd.toml");

        let result = Config::load();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Missing required MQTT_HOST configuration"
        );

        // 2. Test ENV loading
        env::set_var("MQTT_HOST", "127.0.0.1");
        env::set_var("MQTT_PORT", "1883");
        env::set_var("MQTT_USER", "user");
        env::set_var("MQTT_PASSWORD", "secret");
        env::set_var("MQTT_TOPIC_PREFIX", "homeassistant_test");

        let config = Config::load().unwrap();
        assert_eq!(config.mqtt_host, "127.0.0.1");
        assert_eq!(config.mqtt_port, 1883);
        assert_eq!(config.mqtt_user, Some("user".to_string()));
        assert_eq!(config.mqtt_password, Some("secret".to_string()));
        assert_eq!(config.mqtt_topic_prefix, "homeassistant_test");

        // Cleanup env for next step
        env::remove_var("MQTT_HOST");
        env::remove_var("MQTT_PORT");
        env::remove_var("MQTT_USER");
        env::remove_var("MQTT_PASSWORD");
        env::remove_var("MQTT_TOPIC_PREFIX");

        // 3. Test TOML fallback
        let mut file = File::create("sysmqttd.toml").unwrap();
        writeln!(
            file,
            r#"
            # Sample config
            host = "192.168.1.100"
            port = 8883
            user = "toml_user"
            password = "toml_password"
            prefix = "toml_prefix"
            "#
        )
        .unwrap();

        let config_toml = Config::load().unwrap();
        assert_eq!(config_toml.mqtt_host, "192.168.1.100");
        assert_eq!(config_toml.mqtt_port, 8883);
        assert_eq!(config_toml.mqtt_user, Some("toml_user".to_string()));
        assert_eq!(config_toml.mqtt_password, Some("toml_password".to_string()));
        assert_eq!(config_toml.mqtt_topic_prefix, "toml_prefix");

        // 4. Test ENV overriding TOML
        env::set_var("MQTT_HOST", "10.0.0.1");
        env::set_var("MQTT_PORT", "1883");
        let config_overridden = Config::load().unwrap();
        assert_eq!(config_overridden.mqtt_host, "10.0.0.1");
        assert_eq!(config_overridden.mqtt_port, 1883);
        assert_eq!(config_overridden.mqtt_user, Some("toml_user".to_string())); // Stays same from TOML

        // Cleanup
        let _ = fs::remove_file("sysmqttd.toml");
        env::remove_var("MQTT_HOST");
        env::remove_var("MQTT_PORT");
    }
}
