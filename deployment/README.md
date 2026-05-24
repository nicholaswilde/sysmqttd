# Deployment Guide: `sysmqttd`

This guide explains how to install, configure, and manage the `sysmqttd` system monitoring daemon on target ARMv6 single-board computers (specifically Raspberry Pi Zero W running DietPi).

## 1. Installation Requirements
Ensure the following directories are set up and owned by the deployment user (e.g. `dietpi`):
*   Binary location: `/usr/bin/sysmqttd`
*   Configuration folder: `/etc/sysmqttd`
*   Working directory (for local TOML checks): `/var/lib/sysmqttd`

## 2. Step-by-Step Installation

### Step 2.1: Transfer the Binary
Cross-compile the optimized release binary on the development workstation:
```bash
cross build --target arm-unknown-linux-gnueabihf --release
```
Transfer the compiled binary (`target/arm-unknown-linux-gnueabihf/release/sysmqttd`) to your target board's `/usr/bin/sysmqttd` path using `scp` or `rsync`:
```bash
scp target/arm-unknown-linux-gnueabihf/release/sysmqttd dietpi@<board-ip>:/tmp/
```
On the target board, move the binary to its final location and make it executable:
```bash
sudo mv /tmp/sysmqttd /usr/bin/sysmqttd
sudo chmod +x /usr/bin/sysmqttd
```

### Step 2.2: Setup Configuration
Create the configuration folder and draft a `sysmqttd.toml` file:
```bash
sudo mkdir -p /etc/sysmqttd
sudo nano /etc/sysmqttd/sysmqttd.toml
```
Insert your MQTT broker details:
```toml
# /etc/sysmqttd/sysmqttd.toml
host = "192.168.1.50"
port = 1883
user = "your_mqtt_username"
password = "your_mqtt_password"
prefix = "homeassistant"
```
Or set up environment variables in `/etc/default/sysmqttd`:
```bash
sudo nano /etc/default/sysmqttd
```
Insert environment variables:
```bash
MQTT_HOST=192.168.1.50
MQTT_PORT=1883
MQTT_USER=your_mqtt_username
MQTT_PASSWORD=your_mqtt_password
MQTT_TOPIC_PREFIX=homeassistant
```

### Step 2.3: Set Up systemd Service
Copy the systemd unit file:
```bash
sudo cp deployment/sysmqttd.service /etc/systemd/system/sysmqttd.service
```
Reload the systemd daemon to pick up the new service:
```bash
sudo systemctl daemon-reload
```
Enable the service to start automatically on system boot:
```bash
sudo systemctl enable sysmqttd.service
```
Start the service immediately:
```bash
sudo systemctl start sysmqttd.service
```

## 3. Operations & Maintenance

### Check Service Status
```bash
systemctl status sysmqttd.service
```

### View Daemon Logs (systemd journal)
```bash
journalctl -u sysmqttd.service -f
```

### Restart Service
```bash
sudo systemctl restart sysmqttd.service
```
