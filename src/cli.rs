// CLI argument parsing for sysmqttd

/// Represents the action requested via command line.
#[derive(Debug, PartialEq)]
pub enum CliAction {
    /// Print help/usage information and exit.
    PrintHelp,
    /// Print the version of the binary and exit.
    PrintVersion,
    /// Proceed with normal daemon boot.
    Boot { config_path: Option<String> },
}

/// Parse command line arguments.
///
/// Returns a `CliAction` on success or an error string describing the problem.
pub fn parse_arguments(args: Vec<String>) -> Result<CliAction, String> {
    let mut config_path = None;
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
            unknown => {
                return Err(format!("Unknown argument '{}'", unknown));
            }
        }
    }
    Ok(CliAction::Boot { config_path })
}

/// Returns a short usage string printed for the `--help` flag.
pub fn usage() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        "sysmqttd {ver}\n\n\
Usage: sysmqttd [OPTIONS]\n\
Options:\n\
    -h, --help        Print this help message and exit\n\
    -v, --version     Print version information and exit\n\
    -c, --config      Specify custom path to configuration file (TOML, YAML, JSON)\n\
The daemon connects to an MQTT broker as configured via environment variables or a configuration file.\n",
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
            CliAction::Boot { config_path: None }
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
                config_path: Some("custom_cfg.json".to_string())
            }
        );
    }

    #[test]
    fn test_config_flag_missing_val() {
        let args = vec!["sysmqttd".to_string(), "--config".to_string()];
        assert!(parse_arguments(args).is_err());
    }

    #[test]
    fn test_unknown_flag() {
        let args = vec!["sysmqttd".to_string(), "--foo".to_string()];
        assert!(parse_arguments(args).is_err());
    }
}
