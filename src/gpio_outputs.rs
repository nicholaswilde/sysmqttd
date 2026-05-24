use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct GpioOutputConfig {
    pub pin: u32,
    pub name: String,
}

pub fn parse_gpio_outputs_env(val: &str) -> Vec<GpioOutputConfig> {
    val.split(',')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .filter_map(|part| {
            let subparts: Vec<&str> = part.split(':').collect();
            if subparts.len() >= 2 {
                if let Ok(pin) = subparts[0].parse::<u32>() {
                    let name = subparts[1].trim().to_string();
                    return Some(GpioOutputConfig { pin, name });
                }
            }
            None
        })
        .collect()
}

pub struct GpioOutputController {
    pub pin: u32,
    pub name: String,
    base_path: PathBuf,
}

impl GpioOutputController {
    pub fn new(pin: u32, name: String) -> Self {
        Self {
            pin,
            name,
            base_path: PathBuf::from("/sys/class/gpio"),
        }
    }

    pub fn with_base_path(pin: u32, name: String, base_path: PathBuf) -> Self {
        Self {
            pin,
            name,
            base_path,
        }
    }

    pub fn pin_dir(&self) -> PathBuf {
        self.base_path.join(format!("gpio{}", self.pin))
    }

    pub fn export(&self) -> io::Result<()> {
        let export_path = self.base_path.join("export");
        let pin_dir = self.pin_dir();
        if !pin_dir.exists() {
            fs::write(&export_path, self.pin.to_string())?;
        }
        Ok(())
    }

    pub fn unexport(&self) -> io::Result<()> {
        let unexport_path = self.base_path.join("unexport");
        let pin_dir = self.pin_dir();
        if pin_dir.exists() {
            fs::write(&unexport_path, self.pin.to_string())?;
        }
        Ok(())
    }

    pub fn configure_direction(&self) -> io::Result<()> {
        let dir_path = self.pin_dir().join("direction");
        fs::write(&dir_path, "out")?;
        Ok(())
    }

    pub fn write_value(&self, val: u8) -> io::Result<()> {
        let val_path = self.pin_dir().join("value");
        fs::write(&val_path, val.to_string())?;
        Ok(())
    }

    pub fn read_value(&self) -> io::Result<u8> {
        let val_path = self.pin_dir().join("value");
        let content = fs::read_to_string(&val_path)?;
        let val = content.trim().parse::<u8>().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid GPIO value: {}", e),
            )
        })?;
        Ok(val)
    }

    pub fn setup(&self) -> io::Result<()> {
        self.export()?;
        self.configure_direction()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpio_output_controller_mock() {
        let temp_dir = std::env::temp_dir().join("sysmqttd_gpio_output_tests");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let pin = 24;
        let controller = GpioOutputController::with_base_path(
            pin,
            "Mock Relay".to_string(),
            temp_dir.clone(),
        );

        controller.export().unwrap();
        let export_content = fs::read_to_string(temp_dir.join("export")).unwrap();
        assert_eq!(export_content, "24");

        let pin_dir = temp_dir.join("gpio24");
        fs::create_dir_all(&pin_dir).unwrap();

        controller.export().unwrap(); // should do nothing since pin_dir exists

        controller.configure_direction().unwrap();
        let dir_content = fs::read_to_string(pin_dir.join("direction")).unwrap();
        assert_eq!(dir_content, "out");

        fs::write(pin_dir.join("value"), "0\n").unwrap();
        assert_eq!(controller.read_value().unwrap(), 0);

        controller.write_value(1).unwrap();
        let val_content = fs::read_to_string(pin_dir.join("value")).unwrap();
        assert_eq!(val_content, "1");
        assert_eq!(controller.read_value().unwrap(), 1);

        controller.unexport().unwrap();
        let unexport_content = fs::read_to_string(temp_dir.join("unexport")).unwrap();
        assert_eq!(unexport_content, "24");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_parse_gpio_outputs_env() {
        let env_str = "24:Relay 1, 25:LED Indicator";
        let parsed = parse_gpio_outputs_env(env_str);
        assert_eq!(parsed.len(), 2);

        assert_eq!(parsed[0].pin, 24);
        assert_eq!(parsed[0].name, "Relay 1");

        assert_eq!(parsed[1].pin, 25);
        assert_eq!(parsed[1].name, "LED Indicator");

        assert!(parse_gpio_outputs_env("").is_empty());
        assert!(parse_gpio_outputs_env("invalid").is_empty());
        assert!(parse_gpio_outputs_env("abc:name").is_empty());
    }
}
