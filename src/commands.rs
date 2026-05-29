use std::process::Command;
use std::str::FromStr;

/// Represents a whitelisted safe remote action that can be triggered via MQTT.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteAction {
    Reboot,
    Shutdown,
    RestartService,
    StartSystemdService(String),
    StopSystemdService(String),
    RestartSystemdService(String),
}

impl FromStr for RemoteAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "reboot" => Ok(RemoteAction::Reboot),
            "shutdown" => Ok(RemoteAction::Shutdown),
            "restart_service" => Ok(RemoteAction::RestartService),
            invalid => Err(format!("Invalid remote command: '{}'", invalid)),
        }
    }
}

impl RemoteAction {
    /// Maps each action to its corresponding system command and arguments.
    pub fn get_command_and_args(&self) -> (String, Vec<String>) {
        match self {
            RemoteAction::Reboot => ("sudo".to_string(), vec!["reboot".to_string()]),
            RemoteAction::Shutdown => ("sudo".to_string(), vec!["poweroff".to_string()]),
            RemoteAction::RestartService => (
                "sudo".to_string(),
                vec![
                    "systemctl".to_string(),
                    "restart".to_string(),
                    "sysmqttd".to_string(),
                ],
            ),
            RemoteAction::StartSystemdService(svc) => (
                "sudo".to_string(),
                vec!["systemctl".to_string(), "start".to_string(), svc.clone()],
            ),
            RemoteAction::StopSystemdService(svc) => (
                "sudo".to_string(),
                vec!["systemctl".to_string(), "stop".to_string(), svc.clone()],
            ),
            RemoteAction::RestartSystemdService(svc) => (
                "sudo".to_string(),
                vec!["systemctl".to_string(), "restart".to_string(), svc.clone()],
            ),
        }
    }

    /// Executes the safe remote action using system command utilities.
    pub fn execute(&self) -> Result<(), String> {
        let (cmd, args) = self.get_command_and_args();
        let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        Self::execute_command(&cmd, args_ref)
    }

    /// Helper to execute the system command, running "echo" instead under tests.
    fn execute_command(cmd: &str, args: Vec<&str>) -> Result<(), String> {
        let actual_cmd = if cfg!(test) || std::env::var("SYSMQTTD_TEST_ENV").is_ok() {
            if cmd == "non_existent_command_12345" {
                cmd
            } else {
                "echo"
            }
        } else {
            cmd
        };

        let output = Command::new(actual_cmd)
            .args(&args)
            .output()
            .map_err(|e| format!("Failed to spawn command '{}': {}", actual_cmd, e))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!(
                "Command '{} {:?}' exited with status {}: {}",
                actual_cmd, args, output.status, stderr
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_valid_commands() {
        assert_eq!(
            "reboot".parse::<RemoteAction>().unwrap(),
            RemoteAction::Reboot
        );
        assert_eq!(
            "shutdown".parse::<RemoteAction>().unwrap(),
            RemoteAction::Shutdown
        );
        assert_eq!(
            "restart_service".parse::<RemoteAction>().unwrap(),
            RemoteAction::RestartService
        );
    }

    #[test]
    fn test_parsing_case_insensitivity_and_whitespace() {
        assert_eq!(
            "  ReBoOt  ".parse::<RemoteAction>().unwrap(),
            RemoteAction::Reboot
        );
        assert_eq!(
            "SHUTDOWN\n".parse::<RemoteAction>().unwrap(),
            RemoteAction::Shutdown
        );
        assert_eq!(
            "\trestart_service\r".parse::<RemoteAction>().unwrap(),
            RemoteAction::RestartService
        );
    }

    #[test]
    fn test_parsing_invalid_commands() {
        assert!("rm -rf /".parse::<RemoteAction>().is_err());
        assert!("reboot; rm -rf /".parse::<RemoteAction>().is_err());
        assert!("reboot ".trim().parse::<RemoteAction>().is_ok());
        assert!("invalid_cmd".parse::<RemoteAction>().is_err());
        assert!("".parse::<RemoteAction>().is_err());
    }

    #[test]
    fn test_command_mapping() {
        assert_eq!(
            RemoteAction::Reboot.get_command_and_args(),
            ("sudo".to_string(), vec!["reboot".to_string()])
        );
        assert_eq!(
            RemoteAction::Shutdown.get_command_and_args(),
            ("sudo".to_string(), vec!["poweroff".to_string()])
        );
        assert_eq!(
            RemoteAction::RestartService.get_command_and_args(),
            (
                "sudo".to_string(),
                vec![
                    "systemctl".to_string(),
                    "restart".to_string(),
                    "sysmqttd".to_string()
                ]
            )
        );
        assert_eq!(
            RemoteAction::StartSystemdService("nginx".to_string()).get_command_and_args(),
            (
                "sudo".to_string(),
                vec![
                    "systemctl".to_string(),
                    "start".to_string(),
                    "nginx".to_string()
                ]
            )
        );
        assert_eq!(
            RemoteAction::StopSystemdService("nginx".to_string()).get_command_and_args(),
            (
                "sudo".to_string(),
                vec![
                    "systemctl".to_string(),
                    "stop".to_string(),
                    "nginx".to_string()
                ]
            )
        );
        assert_eq!(
            RemoteAction::RestartSystemdService("nginx".to_string()).get_command_and_args(),
            (
                "sudo".to_string(),
                vec![
                    "systemctl".to_string(),
                    "restart".to_string(),
                    "nginx".to_string()
                ]
            )
        );
    }

    #[test]
    fn test_execute_in_test_env() {
        assert!(RemoteAction::Reboot.execute().is_ok());
        assert!(RemoteAction::Shutdown.execute().is_ok());
        assert!(RemoteAction::RestartService.execute().is_ok());
        assert!(RemoteAction::StartSystemdService("nginx".to_string())
            .execute()
            .is_ok());
        assert!(RemoteAction::StopSystemdService("nginx".to_string())
            .execute()
            .is_ok());
        assert!(RemoteAction::RestartSystemdService("nginx".to_string())
            .execute()
            .is_ok());
    }

    #[test]
    fn test_execute_failure() {
        // Run execute_command with a command that doesn't exist to cover error handling
        let res = RemoteAction::execute_command("non_existent_command_12345", vec![]);
        assert!(res.is_err());
    }
}
