# Deployment Guide: `sysmqttd`

This guide explains how to install, configure, and manage the `sysmqttd` system monitoring daemon on Linux systems (such as ARM single-board computers running DietPi, Raspberry Pi OS, Debian, or Ubuntu).

---

## 1. Installation Requirements

To run securely, `sysmqttd` should execute under its own non-root system user and have limited write permissions.

### Create the system user and directories:
On the target system, run:
```bash
# 1. Create a dedicated system user (no login shell)
sudo useradd -r -s /usr/sbin/nologin sysmqttd

# 2. Create the necessary configuration and state directories
sudo mkdir -p /etc/sysmqttd /var/lib/sysmqttd

# 3. Restrict ownership of these directories to the daemon user
sudo chown -R sysmqttd:sysmqttd /etc/sysmqttd /var/lib/sysmqttd
sudo chmod 750 /etc/sysmqttd /var/lib/sysmqttd
```

---

## 2. Step-by-Step Installation

### Step 2.1: Build and Transfer the Binary
Cross-compile the optimized release binary on your development workstation:
```bash
# Example for Raspberry Pi Zero W (ARMv6)
cross build --target arm-unknown-linux-gnueabihf --release
```
Transfer the compiled binary (`target/arm-unknown-linux-gnueabihf/release/sysmqttd`) to your target board:
```bash
scp target/arm-unknown-linux-gnueabihf/release/sysmqttd pi@<board-ip>:/tmp/
```
On the target board, move the binary to `/usr/bin/` and make it executable:
```bash
sudo mv /tmp/sysmqttd /usr/bin/sysmqttd
sudo chmod +x /usr/bin/sysmqttd
sudo chown root:root /usr/bin/sysmqttd
```

### Step 2.2: Set Up Configuration
Create a config file in `/etc/sysmqttd/sysmqttd.toml`:
```bash
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
interface = "wlan0"
```
Ensure the daemon user can read the config file:
```bash
sudo chown root:sysmqttd /etc/sysmqttd/sysmqttd.toml
sudo chmod 640 /etc/sysmqttd/sysmqttd.toml
```

Alternatively, you can configure using environment variables in `/etc/default/sysmqttd`:
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
NET_INTERFACE=wlan0
MONITORED_SERVICES=docker,nginx,ssh
```

### Step 2.3: Set Up the systemd Service
Generate your active systemd service file from the provided template:

```bash
# Replace placeholders to target the 'sysmqttd' user
sed -e 's|{{SYSMQTTD_USER}}|sysmqttd|g' \
    -e 's|{{SYSMQTTD_GROUP}}|sysmqttd|g' \
    -e 's|{{SYSMQTTD_VAR_DIR}}|/var/lib/sysmqttd|g' \
    -e 's|{{SYSMQTTD_BIN}}|/usr/bin/sysmqttd|g' \
    -e 's|{{SYSMQTTD_CONF_FILE}}|/etc/sysmqttd/sysmqttd.toml|g' \
    deployment/sysmqttd.service.template | sudo tee /etc/systemd/system/sysmqttd.service
```

Reload the systemd daemon, enable, and start the service:
```bash
sudo systemctl daemon-reload
sudo systemctl enable sysmqttd.service
sudo systemctl start sysmqttd.service
```

---

## 3. Remote Action Setup (Optional)

If you wish to allow remote commands (e.g. `reboot`, `shutdown`, `restart_service`) via MQTT, the non-root daemon user must be permitted to execute specific system commands via `sudo` without entering a password.

### Step 3.1: Configure passwordless sudo for sysmqttd
Copy the sudoers template and replace the `{{SYSMQTTD_USER}}` placeholder:
```bash
sed 's|{{SYSMQTTD_USER}}|sysmqttd|g' deployment/sysmqttd.sudoers.template | sudo tee /etc/sudoers.d/sysmqttd
```

Secure the file permissions (critical; systemd and sudo will ignore this file if permissions are too broad):
```bash
sudo chmod 0440 /etc/sudoers.d/sysmqttd
sudo chown root:root /etc/sudoers.d/sysmqttd
```

### Step 3.2: Verify systemd Sandbox Capabilities
Make sure Case A in `/etc/systemd/system/sysmqttd.service` is active (which is the default in the template) so that systemd does not strip the setuid capability needed by `sudo`:
```ini
NoNewPrivileges=false
CapabilityBoundingSet=CAP_SETUID CAP_SETGID
```

---

## 4. Operations & Maintenance

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

---

## 5. Local Packaging & Manual Verification (FPM)

For testing and local package generation, you can build `.deb` and `.rpm` files using **FPM (Effing Package Management)**.

### Prerequisites
On Debian/Ubuntu, install `ruby`, `rpm`, and `fpm`:
```bash
sudo apt-get update
sudo apt-get install -y ruby-dev rpm build-essential
sudo gem install --no-document fpm
```

### Staging the Files
Compile the binary for your target or host architecture:
```bash
cargo build --release
```

Create a staging directory mirroring the final system path layout:
```bash
staging="staging-pkg"
mkdir -p "$staging/usr/bin"
mkdir -p "$staging/etc/sysmqttd"
mkdir -p "$staging/usr/share/sysmqttd"

# Copy binary, configs, and systemd templates into staging
cp target/release/sysmqttd "$staging/usr/bin/sysmqttd"
cp sysmqttd.toml.example "$staging/etc/sysmqttd/sysmqttd.toml.example"
cp deployment/sysmqttd.service.template "$staging/usr/share/sysmqttd/sysmqttd.service.template"
cp deployment/sysmqttd.sudoers.template "$staging/usr/share/sysmqttd/sysmqttd.sudoers.template"
```

### Build DEB Package
```bash
fpm -s dir -t deb \
  -n sysmqttd \
  -v "0.1.0" \
  -a "amd64" \
  --description "Lightweight MQTT system telemetry daemon for single-board computers" \
  --maintainer "Nicholas Wilde <https://github.com/nicholaswilde/>" \
  -d "sudo" -d "systemd" \
  --post-install deployment/post_install.sh \
  --pre-uninstall deployment/pre_uninstall.sh \
  -p "sysmqttd-0.1.0-amd64.deb" \
  -C "$staging" .
```

### Build RPM Package
```bash
fpm -s dir -t rpm \
  -n sysmqttd \
  -v "0.1.0" \
  -a "x86_64" \
  --description "Lightweight MQTT system telemetry daemon for single-board computers" \
  --maintainer "Nicholas Wilde <https://github.com/nicholaswilde/>" \
  -d "sudo" -d "systemd" \
  --post-install deployment/post_install.sh \
  --pre-uninstall deployment/pre_uninstall.sh \
  -p "sysmqttd-0.1.0-x86_64.rpm" \
  -C "$staging" .
```

