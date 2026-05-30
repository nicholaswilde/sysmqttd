# :package: sysmqttd

[![Coveralls](https://img.shields.io/coveralls/github/nicholaswilde/sysmqttd/main?style=for-the-badge&logo=coveralls)](https://coveralls.io/github/nicholaswilde/sysmqttd?branch=main)
[![task](https://img.shields.io/badge/Task-Enabled-brightgreen?style=for-the-badge&logo=task&logoColor=white)](https://taskfile.dev/#/)
[![ci](https://img.shields.io/github/actions/workflow/status/nicholaswilde/sysmqttd/ci.yml?label=ci&style=for-the-badge&branch=main&logo=github-actions)](https://github.com/nicholaswilde/sysmqttd/actions/workflows/ci.yml)

> [!WARNING]
> **This is a development version at version v0.1.18 and details may change at any time.**

`sysmqttd` is a lightweight, native Rust system monitoring daemon optimized aggressively for low-resource single-board computers (specifically the Raspberry Pi Zero W running DietPi/Linux). It collects detailed system telemetry and monitors the status of whitelisted systemd services, then streams them to a Home Assistant MQTT broker using Home Assistant MQTT Discovery on startup for seamless, zero-configuration integration.

---

## :star: Features

- **Negligible Footprint:** Optimized native Rust binary under **530KB** when stripped, consuming only **~4-6MB RAM RSS** during active execution.
- **Comprehensive Telemetry:** Gathers CPU Temperature, CPU Usage (%), RAM Usage (%) and absolute capacity (Used & Free in MB), Disk Storage Utilization (%) and absolute capacity (Used & Free in GB), CPU Load Averages (1m, 5m, 15m), System Uptime, Real-time Network Bandwidth Rates (RX & TX rate in kB/s), Primary Interface IP & MAC addresses, Wi-Fi Signal Strength (RSSI in dBm), pending system package upgrades (upgradable package count), and active top resource-consuming process details.
- **Service Status & Control Monitoring:** Asynchronously monitors a customizable whitelist of systemd services (e.g., `docker`, `nginx`, `ssh`), reporting their status as Home Assistant binary sensors (`connectivity` class) and registering corresponding dynamic switch entities for remote control (Start, Stop, Restart).
- **GPIO Input Monitoring:** Monitors physical state transitions of configured GPIO input pins (e.g., buttons, door sensors) and publishes changes instantly as Home Assistant binary sensors.
- **GPIO Output Actuation Control:** Drives physical output devices (e.g., relays, indicators) connected to whitelisted systemd GPIO pins via incoming MQTT switch commands.
- **Native Buttons & Safe Remote Commands:** Registers dedicated native Home Assistant Button entities for system `reboot` and `shutdown` for seamless dashboard interaction, and securely executes whitelisted system controls (`reboot`, `shutdown`, `restart_service`) via a dedicated MQTT subscription topic.
- **Zero-Configuration Auto-Discovery:** Registers all collected telemetry and monitored services/pins under a single parent device in Home Assistant using standard MQTT Discovery.
- **Asynchronous Loop:** Built on the Tokio runtime, featuring isolated, non-blocking telemetry and service monitoring loops.
- **Hardened Deployment:** Comes with a secure systemd unit template utilizing Linux sandboxing technologies for tight security constraints.

---

## :arrow_down: Installation

### Debian / Ubuntu (.deb)

Download the `.deb` package from the [GitHub Releases](https://github.com/nicholaswilde/sysmqttd/releases) for your architecture and install it:

```bash
sudo dpkg -i sysmqttd-*.deb
```

The package installation automatically:
- Creates a dedicated `sysmqttd` system user and group.
- Provisions and secures configuration and state directories (`/etc/sysmqttd`, `/var/lib/sysmqttd`).
- Provisions, secures, and registers the active systemd service unit.

### RedHat / CentOS / Fedora / Rocky Linux (.rpm)

Download the `.rpm` package from the [GitHub Releases](https://github.com/nicholaswilde/sysmqttd/releases) for your architecture and install it:

```bash
sudo rpm -ivh sysmqttd-*.rpm
```

### Homebrew

You can install `sysmqttd` using [Homebrew](https://brew.sh/) via the [nicholaswilde/homebrew-tap](https://github.com/nicholaswilde/homebrew-tap) tap:

```bash
# Add the Homebrew tap
brew tap nicholaswilde/tap

# Install sysmqttd
brew install sysmqttd
```

Alternatively, you can install it directly with:

```bash
brew install nicholaswilde/tap/sysmqttd
```

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

## :computer: CLI Usage

The binary supports the following command-line flags, processed before any configuration loading or MQTT connection is initiated:

- `-h`, `--help` – Show usage information and exit.
- `-v`, `--version` – Show the current version (e.g., `sysmqttd v0.1.0`) and exit.
- `-k`, `--healthcheck` – Run ephemeral diagnostic checks and exit.
- `-c`, `--config <path>` – Specify custom path to a configuration file (TOML, YAML, or JSON).
- `-H`, `--host <host>` – MQTT broker host (e.g., `localhost`).
- `-P`, `--port <port>` – MQTT broker port (default `1883`, or `8883` if TLS is active).
- `-u`, `--user <username>` / `--username <username>` – MQTT broker username.
- `-w`, `--password <pass>` / `--pass <pass>` – MQTT broker password.
- `-p`, `--prefix <prefix>` – Home Assistant discovery topic prefix (default `homeassistant`).
- `-i`, `--interface <if>` – Network interface card to monitor (default `wlan0`).
- `-s`, `--services <list>` / `--monitored-services <list>` – Comma-separated whitelist of systemd services to monitor.
- `-g`, `--gpio <list>` / `--gpio-inputs <list>` – Comma-separated list of GPIO input pins in `pin:name[:device_class]` format.
- `-o`, `--gpio-outputs <list>` – Comma-separated list of GPIO output pins in `pin:name` format.
- `-T`, `-U`, `--unit <unit>` / `--temperature-unit <unit>` – Temperature unit selection (`C` or `F`, case-insensitive, default `F`).
- `--verbose` – Enable verbose logging (detailed payloads, events, and transitions).
- `--tls`, `--use-tls` – Enable secure TLS broker connection (defaults port to `8883`).
- `--ca`, `--ca-cert-path <path>` – Custom/self-signed root CA certificate path for TLS.

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
| `SYSMQTTD_GPIO_INPUTS`   | `GPIO_INPUTS`   | `gpio_inputs` | *None* | Whitelist of GPIO input pins in `pin:name[:device_class]` format. |
| `SYSMQTTD_GPIO_OUTPUTS`  | `GPIO_OUTPUTS`  | `gpio_outputs` | *None* | Whitelist of GPIO output pins in `pin:name` format. |
| `SYSMQTTD_TEMPERATURE_UNIT` | `TEMPERATURE_UNIT` | `temperature_unit` (`unit`, `temp_unit`) | `F` | Temperature unit selection: `C` (Celsius) or `F` (Fahrenheit, default). |
| `SYSMQTTD_VERBOSE`       | *N/A*           | `verbose` | `false` | Enable verbose logging (detailed payloads, events, and transitions). |
| `SYSMQTTD_USE_TLS`       | `USE_TLS`       | `use_tls` (`tls`) | `false` | Enable secure TLS broker connection (defaults port to `8883`). |
| `SYSMQTTD_CA_CERT_PATH`  | `CA_CERT_PATH`  | `ca_cert_path` (`ca_path`) | *None* | Optional custom/self-signed root CA certificate path for TLS. |
| `SYSMQTTD_RECONNECT_INITIAL_DELAY` | `RECONNECT_INITIAL_DELAY` | `reconnect_initial_delay` (`initial_delay`) | `2` | Initial connection retry delay in seconds before doubling on consecutive failures. |
| `SYSMQTTD_RECONNECT_MAX_DELAY` | `RECONNECT_MAX_DELAY` | `reconnect_max_delay` (`max_delay`) | `300` | Maximum connection retry delay ceiling in seconds. |
| `SYSMQTTD_SD_ALERT_THRESHOLD` | `SD_ALERT_THRESHOLD` | `sd_alert_threshold` (`sd_threshold`) | `95.0` | Root disk space utilization threshold percentage that triggers a Home Assistant alert and dynamically silences logging to prevent disk-write amplification loops. |
### Sample Configuration Files

A fully documented, production-ready configuration template is available in the root of the repository as [sysmqttd.toml.example](file:///home/nicholas/git/nicholaswilde/sysmqttd/sysmqttd.toml.example). You can copy this file to get started quickly:

```bash
cp sysmqttd.toml.example sysmqttd.toml
```

You can write your configuration file in TOML, YAML, or JSON. Here are examples for each supported format:

#### TOML (`sysmqttd.toml`)
```toml
host = "192.168.1.50"
port = 1883
user = "mqtt_user"
password = "supersecretpassword"
prefix = "homeassistant"
interface = "eth0"
verbose = false
temperature_unit = "F"
sd_alert_threshold = 95.0

gpio_inputs = [
  { pin = 23, name = "Front Door", device_class = "door" }
]

gpio_outputs = [
  { pin = 24, name = "Relay 1" }
]
```

#### YAML (`sysmqttd.yaml` / `sysmqttd.yml`)
```yaml
host: "192.168.1.50"
port: 1883
user: "mqtt_user"
password: "supersecretpassword"
prefix: "homeassistant"
interface: "eth0"
verbose: false
temperature_unit: "F"
sd_alert_threshold: 95.0
gpio_inputs:
  - pin: 23
    name: "Front Door"
    device_class: "door"
gpio_outputs:
  - pin: 24
    name: "Relay 1"
```

#### JSON (`sysmqttd.json`)
```json
{
  "host": "192.168.1.50",
  "port": 1883,
  "user": "mqtt_user",
  "password": "supersecretpassword",
  "prefix": "homeassistant",
  "interface": "eth0",
  "verbose": false,
  "temperature_unit": "F",
  "sd_alert_threshold": 95.0,
  "gpio_inputs": [
    { "pin": 23, "name": "Front Door", "device_class": "door" }
  ],
  "gpio_outputs": [
    { "pin": 24, "name": "Relay 1" }
  ]
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
SYSMQTTD_MQTT_HOST=192.168.1.50
SYSMQTTD_MQTT_PORT=1883
SYSMQTTD_MQTT_USER=mqtt_user
SYSMQTTD_MQTT_PASSWORD=supersecretpassword
SYSMQTTD_MQTT_TOPIC_PREFIX=homeassistant
SYSMQTTD_NET_INTERFACE=eth0
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

Once `sysmqttd` starts and establishes a connection, it automatically broadcasts MQTT Discovery payloads to register all sensors under a single parent device, which will show up in Home Assistant as **`sysmqtt <hostname>`**.

### Availability Topic & Last Will

`sysmqttd` utilizes Birth and Last Will & Testament messages to maintain accurate sensor availability in Home Assistant.

- **Availability Topic:** `<MQTT_TOPIC_PREFIX>/sensor/sysmqttd_<hostname>/availability`
- **Behavior:**
  - On a successful connection, a birth message of `online` is published.
  - If the daemon shuts down gracefully or loses its connection to the broker unexpectedly, the broker automatically publishes the last will message of `offline`.

### Telemetry State Topic

System telemetry metrics are parsed and published every **60 seconds** in a single, flat JSON payload to minimize network traffic and processing overhead.

- **Telemetry State Topic:** `<MQTT_TOPIC_PREFIX>/sensor/sysmqttd_<hostname>/state`
- **CPU Temperature Unit:** The `cpu_temperature` value is automatically converted and published in Celsius (`°C`) or Fahrenheit (`°F`, default) depending on your configuration.
- **Telemetry Payload Example:**
  ```json
  {
    "cpu_temperature": 43.5,
    "ram_usage": 14.8,
    "ram_used_mb": 151.6,
    "ram_free_mb": 872.4,
    "disk_usage": 32.4,
    "disk_used_gb": 4.8,
    "disk_free_gb": 10.2,
    "cpu_usage": 12.5,
    "load_1m": 0.05,
    "load_5m": 0.12,
    "load_15m": 0.08,
    "uptime_seconds": 154320.0,
    "net_rx_rate": 42.7,
    "net_tx_rate": 12.3,
    "upgradable_packages": 3,
    "top_process": "firefox (12345) - 12.5% CPU, 412.3 MB RAM"
  }
  ```

### Uptime Telemetry & Home Assistant Conversion

The `uptime_seconds` metric is published as a raw float representing the total system uptime in seconds (e.g., `154320.0`). 

By default, Home Assistant will display this as a raw number of seconds. If you want to convert this into a human-readable duration (e.g., days, hours, minutes) or a relative time, you can create a **Template Sensor** in your Home Assistant `configuration.yaml` file:

```yaml
template:
  - sensor:
      - name: "Pi Zero Uptime Readable"
        state: >
          {% set uptime = states('sensor.sysmqttd_<hostname>_uptime') | float(0) %}
          {% set days = (uptime / 86400) | int %}
          {% set hours = ((uptime % 86400) / 3600) | int %}
          {% set minutes = ((uptime % 3600) / 60) | int %}
          {{ days }}d {{ hours }}h {{ minutes }}m
```

Alternatively, you can convert it to an absolute boot timestamp to display it as a relative time in the Home Assistant UI:

```yaml
template:
  - sensor:
      - name: "Pi Zero Last Boot Time"
        device_class: timestamp
        state: >
          {{ (as_timestamp(now()) - (states('sensor.sysmqttd_<hostname>_uptime') | float(0))) | timestamp_custom('%Y-%m-%dT%H:%M:%S+00:00', false) }}
```

### Monitored Services Topics

Monitored systemd services are registered as individual binary sensors of the `connectivity` class. Their states are published on a distinct topic path:

- **Service State Topic:** `<MQTT_TOPIC_PREFIX>/binary_sensor/sysmqttd_<hostname>/service_<service_name>/state`
- **Values:** `"on"` (service is running/active) or `"off"` (service is inactive/dead).

Additionally, a dynamic switch entity is created in Home Assistant to control the service's power state:

- **Service Command Topic:** `<MQTT_TOPIC_PREFIX>/switch/sysmqttd_<hostname>_service_<service_name>/set`
- **Values:** `"ON"` (starts the systemd service), `"OFF"` (stops the systemd service), or `"RESTART"` (restarts the systemd service). When a command payload is received, the daemon executes the corresponding command (`systemctl start/stop/restart`) and publishes the updated state back to the service state topic.

### GPIO Input Topics

Monitored GPIO input pins are registered as binary sensors:

- **GPIO Input State Topic:** `<MQTT_TOPIC_PREFIX>/binary_sensor/sysmqttd_<hostname>_pin<pin_number>/state`
- **Values:** `"ON"` (high/1) or `"OFF"` (low/0).

### GPIO Output Switch Topics

Monitored GPIO output pins are registered as switch entities:

- **GPIO Output Command Topic:** `<MQTT_TOPIC_PREFIX>/switch/sysmqttd_<hostname>_pin<pin_number>/set`
- **GPIO Output State Topic:** `<MQTT_TOPIC_PREFIX>/switch/sysmqttd_<hostname>_pin<pin_number>/state`
- **Values:** `"ON"` or `"OFF"`. When a command payload is received, the daemon actuates the pin and confirms the updated state.

### Remote Commands & Buttons

The daemon registers native Home Assistant **Button** entities for system Reboot and Shutdown, allowing one-click control directly from the HA UI. It also subscribes to a dedicated command topic to receive and securely process whitelisted system instructions:

- **Remote Command Topic:** `<MQTT_TOPIC_PREFIX>/sensor/sysmqttd_<hostname>/command`
- **Supported Command Payloads:** 
  - `"reboot"` – Reboots the host system (executes `sudo reboot`). Triggered by the Reboot button.
  - `"shutdown"` – Powers off the host system (executes `sudo poweroff`). Triggered by the Shutdown button.
  - `"restart_service"` – Restarts the `sysmqttd` daemon itself (executes `sudo systemctl restart sysmqttd`).

> [!NOTE]
> **All other command strings, arguments, or shell flags are completely ignored and discarded to prevent command injection.**

### Dynamic Telemetry Polling Control

`sysmqttd` supports dynamic, real-time adjustments of its telemetry polling interval during execution via incoming MQTT messages, allowing you to change how frequently the daemon gathers and publishes system stats.

- **Command Topic:** `<MQTT_TOPIC_PREFIX>/sensor/sysmqttd_<hostname>/interval/set`
- **State Topic:** `<MQTT_TOPIC_PREFIX>/sensor/sysmqttd_<hostname>/interval/state`
- **Payload Requirements:** An integer value between `1` and `86400` (representing interval in seconds, i.e., up to 24 hours).
- **Validation:** Payloads that are out of bounds or non-numeric are rejected, and the daemon retains its previous interval.
- **Home Assistant Autodiscovery:** The interval registers as a **Number** entity (`Telemetry Interval`) in Home Assistant, allowing you to control the polling interval directly using a slider in the Home Assistant UI.

### Auto-Discovery Topics

For every metric and monitored service, discovery configurations are published so they auto-register in Home Assistant:

- **Telemetry Interval Discovery Config Topic:**
  `homeassistant/number/sysmqttd_<hostname>_interval/config`
- **CPU Temperature Discovery Config Topic:**
  `homeassistant/sensor/sysmqttd_<hostname>_cpu_temp/config`
- **CPU Usage Discovery Config Topic:**
  `homeassistant/sensor/sysmqttd_<hostname>_cpu_usage/config`
- **RAM Used Discovery Config Topic:**
  `homeassistant/sensor/sysmqttd_<hostname>_ram_used/config`
- **Disk Used Discovery Config Topic:**
  `homeassistant/sensor/sysmqttd_<hostname>_disk_used/config`
- **System Uptime Discovery Config Topic:**
  `homeassistant/sensor/sysmqttd_<hostname>_uptime/config`
- **Network RX Rate Discovery Config Topic:**
  `homeassistant/sensor/sysmqttd_<hostname>_net_rx_rate/config`
- **Upgradable Packages Discovery Config Topic:**
  `homeassistant/sensor/sysmqttd_<hostname>_upgradable_packages/config`
- **Top Process Discovery Config Topic:**
  `homeassistant/sensor/sysmqttd_<hostname>_top_process/config`
- **Monitored Service (e.g. `nginx`) Discovery Config Topic:**
  `homeassistant/binary_sensor/sysmqttd_<hostname>_service_nginx/config`
- **Reboot Button Discovery Config Topic:**
  `homeassistant/button/sysmqttd_<hostname>_reboot/config`
- **Shutdown Button Discovery Config Topic:**
  `homeassistant/button/sysmqttd_<hostname>_shutdown/config`
- **Service Control Switch (e.g. `nginx`) Discovery Config Topic:**
  `homeassistant/switch/sysmqttd_<hostname>_service_nginx/config`

Each discovery payload specifies the JSON extraction path via the `"value_template"` configuration (e.g. `"{{ value_json.cpu_temperature }}"`), ensuring zero custom YAML configuration is required on the Home Assistant side.

### :bar_chart: Coverage

The project uses `cargo-llvm-cov` for code coverage analysis.

```bash
# Show coverage summary in console
task coverage
```

## :handshake: Contributing

Contributions are welcome! Please follow standard Rust coding conventions and ensure all tests pass (`task test:ci`) before submitting features.

## :balance_scale: License

[Apache License 2.0](LICENSE)

## :writing_hand: Author

This project was started in 2026 by [Nicholas Wilde](https://github.com/nicholaswilde/).
