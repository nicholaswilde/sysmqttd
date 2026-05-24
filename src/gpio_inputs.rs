use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct GpioInputConfig {
    pub pin: u32,
    pub name: String,
    #[serde(default)]
    pub device_class: Option<String>,
}

pub fn parse_gpio_inputs_env(val: &str) -> Vec<GpioInputConfig> {
    val.split(',')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .filter_map(|part| {
            let subparts: Vec<&str> = part.split(':').collect();
            if subparts.len() >= 2 {
                if let Ok(pin) = subparts[0].parse::<u32>() {
                    let name = subparts[1].trim().to_string();
                    let device_class = if subparts.len() >= 3 {
                        let dc = subparts[2].trim().to_string();
                        if dc.is_empty() {
                            None
                        } else {
                            Some(dc)
                        }
                    } else {
                        None
                    };
                    return Some(GpioInputConfig {
                        pin,
                        name,
                        device_class,
                    });
                }
            }
            None
        })
        .collect()
}

pub struct GpioInputListener {
    pub pin: u32,
    pub name: String,
    pub device_class: Option<String>,
    base_path: PathBuf,
    last_value: Option<u8>,
}

impl GpioInputListener {
    pub fn new(pin: u32, name: String, device_class: Option<String>) -> Self {
        Self {
            pin,
            name,
            device_class,
            base_path: PathBuf::from("/sys/class/gpio"),
            last_value: None,
        }
    }

    pub fn with_base_path(
        pin: u32,
        name: String,
        device_class: Option<String>,
        base_path: PathBuf,
    ) -> Self {
        Self {
            pin,
            name,
            device_class,
            base_path,
            last_value: None,
        }
    }

    pub fn pin_dir(&self) -> PathBuf {
        self.base_path.join(format!("gpio{}", self.pin))
    }

    pub fn export(&self) -> io::Result<()> {
        let export_path = self.base_path.join("export");
        let pin_dir = self.pin_dir();
        if !pin_dir.exists() {
            // Write pin to export
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
        fs::write(&dir_path, "in")?;
        Ok(())
    }

    pub fn configure_edge(&self, edge: &str) -> io::Result<()> {
        let edge_path = self.pin_dir().join("edge");
        fs::write(&edge_path, edge)?;
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

    /// Initialize the GPIO pin (export, configure direction, configure edge, and read initial value)
    pub fn setup(&mut self) -> io::Result<()> {
        self.export()?;
        self.configure_direction()?;
        self.configure_edge("both")?;
        if let Ok(v) = self.read_value() {
            self.last_value = Some(v);
        }
        Ok(())
    }

    /// Read the pin value and return a boolean state transition if it changed.
    /// Returns `Some(current_value)` if it changed since last check (or if it is the first check).
    /// Otherwise returns `None`.
    pub fn check_transition(&mut self) -> io::Result<Option<u8>> {
        let val = self.read_value()?;
        match self.last_value {
            Some(last) if last == val => Ok(None),
            _ => {
                self.last_value = Some(val);
                Ok(Some(val))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpio_input_listener_mock() {
        let temp_dir = std::env::temp_dir().join("sysmqttd_gpio_tests");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let pin = 23;
        let mut listener = GpioInputListener::with_base_path(
            pin,
            "Mock Button".to_string(),
            Some("door".to_string()),
            temp_dir.clone(),
        );

        listener.export().unwrap();
        // Verify that export file contains pin
        let export_content = fs::read_to_string(temp_dir.join("export")).unwrap();
        assert_eq!(export_content, "23");

        // Simulate kernel creating the gpio23 directory
        let pin_dir = temp_dir.join("gpio23");
        fs::create_dir_all(&pin_dir).unwrap();

        // Now test export when pin_dir already exists (should do nothing)
        listener.export().unwrap();

        listener.configure_direction().unwrap();
        let dir_content = fs::read_to_string(pin_dir.join("direction")).unwrap();
        assert_eq!(dir_content, "in");

        listener.configure_edge("both").unwrap();
        let edge_content = fs::read_to_string(pin_dir.join("edge")).unwrap();
        assert_eq!(edge_content, "both");

        // Simulate initial value
        fs::write(pin_dir.join("value"), "0\n").unwrap();
        assert_eq!(listener.read_value().unwrap(), 0);

        // Test setup
        listener.setup().unwrap();
        assert_eq!(listener.last_value, Some(0));

        // check_transition: no change
        let t1 = listener.check_transition().unwrap();
        assert_eq!(t1, None);

        // change value to 1
        fs::write(pin_dir.join("value"), "1\n").unwrap();
        let t2 = listener.check_transition().unwrap();
        assert_eq!(t2, Some(1));
        assert_eq!(listener.last_value, Some(1));

        // check_transition: no change
        let t3 = listener.check_transition().unwrap();
        assert_eq!(t3, None);

        // unexport
        listener.unexport().unwrap();
        let unexport_content = fs::read_to_string(temp_dir.join("unexport")).unwrap();
        assert_eq!(unexport_content, "23");

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_parse_gpio_inputs_env() {
        let env_str = "23:Front Door:door,24:Motion Sensor:motion, 25:Simple Button";
        let parsed = parse_gpio_inputs_env(env_str);
        assert_eq!(parsed.len(), 3);

        assert_eq!(parsed[0].pin, 23);
        assert_eq!(parsed[0].name, "Front Door");
        assert_eq!(parsed[0].device_class, Some("door".to_string()));

        assert_eq!(parsed[1].pin, 24);
        assert_eq!(parsed[1].name, "Motion Sensor");
        assert_eq!(parsed[1].device_class, Some("motion".to_string()));

        assert_eq!(parsed[2].pin, 25);
        assert_eq!(parsed[2].name, "Simple Button");
        assert_eq!(parsed[2].device_class, None);

        // Test with empty string / invalid parts
        assert!(parse_gpio_inputs_env("").is_empty());
        assert!(parse_gpio_inputs_env("invalid").is_empty());
        assert!(parse_gpio_inputs_env("abc:name").is_empty());
    }
}
