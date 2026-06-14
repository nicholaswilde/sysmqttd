# Product: `sysmqttd` System Monitor

## Vision
A highly-efficient, lightweight system monitoring daemon for resource-constrained ARMv6 single-board computers (specifically Raspberry Pi Zero W running DietPi). It seamlessly integrates with Home Assistant via MQTT Discovery and provides real-time system metrics (CPU Temp, RAM usage, Disk usage) with a negligible resource footprint.

## Target Audience
System administrators, makers, and smart home enthusiasts running Home Assistant who want to monitor their low-power single-board computers without incurring high memory or CPU overhead.

## Key Features
1.  **Tiny Footprint:** Native compiled Rust binary optimized for size (< 2MB stripped) and memory (< 8MB RAM).
2.  **Home Assistant Autodiscovery:** Zero-configuration setup in Home Assistant via explicit, retained MQTT discovery payloads published on startup.
3.  **Core System Telemetry:** Gathers CPU Temperature, Fan Speeds (RPM), CPU Usage (%), RAM Usage (%) and absolute capacity (Used & Free in MB), Disk Storage Utilization (%) and absolute capacity (Used & Free in GB), CPU Load Averages (1m, 5m, 15m), System Uptime, Real-time Network Bandwidth Rates (RX & TX rate in kB/s), Primary Interface IP & MAC addresses, Wi-Fi Signal Strength (RSSI in dBm), pending system package upgrades (upgradable package count), and active top resource-consuming process details.
4.  **Low Disk Space Safe-Guard:** Configurable root disk capacity monitoring threshold (default 95.0%) that publishes a Home Assistant "SD Card Space Alert" binary sensor (problem device class) and dynamically silences all stdout/stderr logging outputs to prevent journald/syslog disk write amplification loops.
5.  **Robust Async Lifecycle:** Asynchronous client loop that handles system startup, intermittent network dropouts, and broker reconnects gracefully with a dynamic, jittered exponential backoff strategy and milestone-based log throttling to prevent thundering herd conditions.
6.  **Service Status Monitoring & Control:** Real-time checking of whitelisted systemd services (e.g., `docker`, `nginx`) via async polling (registered as binary sensors) and dynamic control switches in Home Assistant allowing safe remote start, stop, and restart commands.
7.  **Layered Configuration:** Supports multi-format configuration files (TOML, YAML, JSON), custom config path overrides via `-c`/`--config` flags, and high-precedence `SYSMQTTD_` prefixed environment variables.
*   **CLI Arguments & Diagnostics:** Supports `-h/--help`, `-v/--version`, `-c/--config`, `-k/--healthcheck` (ephemeral diagnostic healthcheck mode), individual parameter override flags (`-H/--host`, `-P/--port`, `-u/--user/--username`, `-w/--password/--pass`, `-p/--prefix`, `-i/--interface`, `-s/--services/--monitored-services`, `-g/--gpio/--gpio-inputs`, `-o/--gpio-outputs`, `--tls`/`--use-tls`, `--ca`/`--ca-cert-path`, and `--no-fan`), and `--verbose` taking absolute highest precedence.
8.  **GPIO Input Monitoring:** Monitors physical state transitions of system GPIO pins configured as inputs (e.g., buttons, magnetic contact door sensors) and publishes changes instantly as Home Assistant binary sensors (`binary_sensor`) with customizable device classes.
9.  **GPIO Output Control:** Actuates physical output devices (like relays, status LEDs, and buzzers) connected to configured GPIO pins via incoming MQTT switch commands, registering them as standard Home Assistant switch entities.
10. **Safe Remote Commands & Buttons:** Accepts authorized and whitelisted remote system commands (`reboot`, `shutdown`, `restart_service`) via secure MQTT subscriptions, registering Reboot and Shutdown as native Home Assistant button entities to trigger clean host controls under non-root privileges.
11. **Native Packaging:** Available as native multi-architecture Debian (`.deb`) and RedHat (`.rpm`) packages for all supported platforms (`x86_64`, `arm64`, `armhf`, `armel`), featuring automatic systemd service provisioning, dedicated non-root execution user setups, and zero-configuration defaults.
12. **Dynamic Telemetry Polling Control:** Supports real-time, bounds-checked adjustment of the telemetry polling interval (from 1s to 86400s) during execution via incoming MQTT messages without requiring a process restart, autodiscovering as a Home Assistant number entity and publishing confirmed interval state updates.




