// CLI argument parsing for sysmqttd

/// Represents the action requested via command line.
#[derive(Debug, PartialEq)]
pub enum CliAction {
    /// Print help/usage information and exit.
    PrintHelp,
    /// Print the version of the binary and exit.
    PrintVersion,
    /// Proceed with normal daemon boot.
    Boot,
}

/// Parse command line arguments.
///
/// Returns a `CliAction` on success or an error string describing the problem.
pub fn parse_arguments(args: Vec<String>) -> Result<CliAction, String> {
    // The binary name is args[0]; we only care about subsequent flags.
    // If no extra arguments, we boot the daemon.
    if args.len() == 1 {
        return Ok(CliAction::Boot);
    }
    // Examine each argument; we only support a single flag for simplicity.
    // Any additional unknown flag results in an error.
    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "-h" | "--help" => return Ok(CliAction::PrintHelp),
            "-v" | "--version" => return Ok(CliAction::PrintVersion),
            unknown => {
                return Err(format!("Unknown argument '{}'", unknown));
            }
        }
    }
    Ok(CliAction::Boot)
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
The daemon connects to an MQTT broker as configured via environment variables or a TOML file.\n",
        ver = version
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_args_boots() {
        let args = vec!["sysmqttd".to_string()];
        assert_eq!(parse_arguments(args).unwrap(), CliAction::Boot);
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
    fn test_unknown_flag() {
        let args = vec!["sysmqttd".to_string(), "--foo".to_string()];
        assert!(parse_arguments(args).is_err());
    }
}
