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

    pub fn with_base_path(pin: u32, name: String, device_class: Option<String>, base_path: PathBuf) -> Self {
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
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid GPIO value: {}", e))
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
}
