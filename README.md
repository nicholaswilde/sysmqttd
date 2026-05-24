# :package: sysmqttd

> [!WARNING]
> **This is a development version at version v0.1.* and details may change at any time.**

`sysmqttd` is a lightweight, native Rust system monitoring daemon optimized aggressively for low-resource single-board computers (specifically the Raspberry Pi Zero W running DietPi/Linux). It collects detailed system telemetry and monitors the status of whitelisted systemd services, then streams them to a Home Assistant MQTT broker using Home Assistant MQTT Discovery on startup for seamless, zero-configuration integration.

---

## :star: Features

- **Negligible Footprint:** Optimized native Rust binary under **530KB** when stripped, consuming only **~4-6MB RAM RSS** during active execution.
- **Comprehensive Telemetry:** Gathers CPU Temperature, RAM Usage (%), Disk Storage Utilization (%), CPU Load Averages (1m, 5m, 15m), System Uptime, and Real-time Network Bandwidth Rates (RX & TX rate in kB/s).
- **Service Status Monitoring:** Asynchronously monitors a customizable whitelist of systemd services (e.g., `docker`, `nginx`, `ssh`), reporting their status as Home Assistant binary sensors (`connectivity` class).
- **Zero-Configuration Auto-Discovery:** Registers all collected telemetry and monitored services under a single parent device in Home Assistant using standard MQTT Discovery.
- **Asynchronous Loop:** Built on the Tokio runtime, featuring isolated, non-blocking telemetry and service monitoring loops.
- **Hardened Deployment:** Comes with a secure systemd unit template utilizing Linux sandboxing technologies for tight security constraints.

---

## :rocket: Quick Start & Dev Commands

This project uses [go-task](https://taskfile.dev/) as its task runner.

### List Available Tasks
```bash
task --list
```

### Run Local Tests
```bash
task test
```

### Check Code Coverage
```bash
task coverage
```

### Compile Cross-Compiled Binaries
```bash
task build-all
```

---

## :terminal: CLI Usage

The binary supports the following command-line flags, processed before any configuration loading or MQTT connection is initiated:

- `-h`, `--help` – Show usage information and exit.
- `-v`, `--version` – Show the current version (e.g., `sysmqttd v0.1.0`) and exit.
- `-c`, `--config <path>` – Specify custom path to a configuration file (TOML, YAML, or JSON).
- `-H`, `--host <host>` – MQTT broker host (e.g., `localhost`).
- `-P`, `--port <port>` – MQTT broker port (default `1883`).
- `-u`, `--user <username>` / `--username <username>` – MQTT broker username.
- `-w`, `--password <pass>` / `--pass <pass>` – MQTT broker password.
- `-p`, `--prefix <prefix>` – Home Assistant discovery topic prefix (default `homeassistant`).
- `-i`, `--interface <if>` – Network interface card to monitor (default `wlan0`).
- `-s`, `--services <list>` / `--monitored-services <list>` – Comma-separated whitelist of systemd services to monitor.

---

## :gear: Configuration

`sysmqttd` supports configuration through a hierarchical, layered structure with the following precedence (from highest to lowest):

1. **Command Line Arguments** (both custom config path and individual parameter flags)
2. **Prefixed Environment Variables** (`SYSMQTTD_*`)
3. **Legacy Environment Variables** (`MQTT_*`)
4. **Configuration File** (searched in order: TOML, YAML, JSON in the local directory, falling back to `/etc/sysmqttd/`)
5. **Built-in Defaults**

### Configuration Reference

| Prefixed Env Var | Legacy Env Var | File Key (TOML/YAML/JSON) | Default Value | Description |
| :--- | :--- | :--- | :--- | :--- |
| `SYSMQTTD_MQTT_HOST` | `MQTT_HOST` | `mqtt_host` (`host`) | *Required* | Hostname or IP address of the MQTT broker. |
| `SYSMQTTD_MQTT_PORT` | `MQTT_PORT` | `mqtt_port` (`port`) | `1883` | Port for connecting to the MQTT broker. |
| `SYSMQTTD_MQTT_USER` | `MQTT_USER` | `mqtt_user` (`user`, `username`) | *None* | Optional username for broker authentication. |
| `SYSMQTTD_MQTT_PASSWORD` | `MQTT_PASSWORD` | `mqtt_password` (`password`, `pass`) | *None* | Optional password for broker authentication. |
| `SYSMQTTD_MQTT_TOPIC_PREFIX`| `MQTT_TOPIC_PREFIX` | `mqtt_topic_prefix` (`prefix`) | `homeassistant` | Discovery prefix for Home Assistant MQTT topics. |
| `SYSMQTTD_NET_INTERFACE` | `NET_INTERFACE` | `net_interface` (`interface`) | `wlan0` | The network interface to monitor for RX/TX rates. |
| *N/A* (Environment only) | `MONITORED_SERVICES` | *N/A* (Environment only) | *None* | Comma-separated list of systemd services to monitor. |

### Sample Configuration Files

You can write your configuration file in TOML, YAML, or JSON. Here are examples for each supported format:

#### TOML (`sysmqttd.toml`)
```toml
host = "192.168.1.50"
port = 1883
user = "mqtt_user"
password = "supersecretpassword"
prefix = "homeassistant"
interface = "eth0"
```

#### YAML (`sysmqttd.yaml` / `sysmqttd.yml`)
```yaml
host: "192.168.1.50"
port: 1883
user: "mqtt_user"
password: "supersecretpassword"
prefix: "homeassistant"
interface: "eth0"
```

#### JSON (`sysmqttd.json`)
```json
{
  "host": "192.168.1.50",
  "port": 1883,
  "user": "mqtt_user",
  "password": "supersecretpassword",
  "prefix": "homeassistant",
  "interface": "eth0"
}
```


---

## :wrench: Systemd Service Setup

To run `sysmqttd` as a reliable background service under systemd, you can deploy the secure, sandboxed unit template.

### 1. Copy the Service File
Copy the provided unit file to the systemd directory:
```bash
sudo cp deployment/sysmqttd.service /etc/systemd/system/sysmqttd.service
```

### 2. Configure the Service Options
The systemd service loads variables from `/etc/default/sysmqttd`. Create this file and specify your environment configurations:
```bash
sudo nano /etc/default/sysmqttd
```

Provide the required variable assignments (note that `MONITORED_SERVICES` allows you to monitor whitelisted services):
```bash
MQTT_HOST=192.168.1.50
MQTT_PORT=1883
MQTT_USER=mqtt_user
MQTT_PASSWORD=supersecretpassword
MQTT_TOPIC_PREFIX=homeassistant
NET_INTERFACE=eth0
MONITORED_SERVICES=docker,nginx,ssh
```

### 3. Initialize and Start the Service
Reload the systemd daemon, enable the service to boot automatically, and start it immediately:
```bash
sudo systemctl daemon-reload
sudo systemctl enable sysmqttd.service
sudo systemctl start sysmqttd.service
```

### 4. Operations & Monitoring
You can interact with the service using standard `systemctl` commands:

- **Check Service Status:**
  ```bash
  systemctl status sysmqttd.service
  ```
- **Read Real-time Logs (systemd journal):**
  ```bash
  journalctl -u sysmqttd.service -f
  ```
- **Restart the Service:**
  ```bash
  sudo systemctl restart sysmqttd.service
  ```

---

## :house: Home Assistant Integration

Once `sysmqttd` starts and establishes a connection, it automatically broadcasts MQTT Discovery payloads to register all sensors under a single parent device based on your device hostname.

### Availability Topic & Last Will

`sysmqttd` utilizes Birth and Last Will & Testament messages to maintain accurate sensor availability in Home Assistant.

- **Availability Topic:** `<MQTT_TOPIC_PREFIX>/sensor/sysmqttd_<hostname>/availability`
- **Behavior:**
  - On a successful connection, a birth message of `online` is published.
  - If the daemon shuts down gracefully or loses its connection to the broker unexpectedly, the broker automatically publishes the last will message of `offline`.

### Telemetry State Topic

System telemetry metrics are parsed and published every **60 seconds** in a single, flat JSON payload to minimize network traffic and processing overhead.

- **Telemetry State Topic:** `<MQTT_TOPIC_PREFIX>/sensor/sysmqttd_<hostname>/state`
- **Telemetry Payload Example:**
  ```json
  {
    "cpu_temperature": 43.5,
    "ram_usage": 14.8,
    "disk_usage": 32.4,
    "load_1m": 0.05,
    "load_5m": 0.12,
    "load_15m": 0.08,
    "uptime_seconds": 154320.0,
    "net_rx_rate": 42.7,
    "net_tx_rate": 12.3
  }
  ```

### Monitored Services Topics

Monitored systemd services are registered as individual binary sensors of the `connectivity` class. Their states are published on a distinct topic path:

- **Service State Topic:** `<MQTT_TOPIC_PREFIX>/binary_sensor/sysmqttd_<hostname>/service_<service_name>/state`
- **Values:** `"on"` (service is running/active) or `"off"` (service is inactive/dead).

### Auto-Discovery Topics

For every metric and monitored service, discovery configurations are published so they auto-register in Home Assistant:

- **CPU Temperature Discovery Config Topic:**
  `homeassistant/sensor/sysmqttd_<hostname>_cpu_temp/config`
- **Network RX Rate Discovery Config Topic:**
  `homeassistant/sensor/sysmqttd_<hostname>_net_rx_rate/config`
- **Monitored Service (e.g. `nginx`) Discovery Config Topic:**
  `homeassistant/binary_sensor/sysmqttd_<hostname>_service_nginx/config`

Each discovery payload specifies the JSON extraction path via the `"value_template"` configuration (e.g. `"{{ value_json.cpu_temperature }}"`), ensuring zero custom YAML configuration is required on the Home Assistant side.