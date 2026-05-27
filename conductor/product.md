# Product: `sysmqttd` System Monitor

## Vision
A highly-efficient, lightweight system monitoring daemon for resource-constrained ARMv6 single-board computers (specifically Raspberry Pi Zero W running DietPi). It seamlessly integrates with Home Assistant via MQTT Discovery and provides real-time system metrics (CPU Temp, RAM usage, Disk usage) with a negligible resource footprint.

## Target Audience
System administrators, makers, and smart home enthusiasts running Home Assistant who want to monitor their low-power single-board computers without incurring high memory or CPU overhead.

## Key Features
1.  **Tiny Footprint:** Native compiled Rust binary optimized for size (< 2MB stripped) and memory (< 8MB RAM).
2.  **Home Assistant Autodiscovery:** Zero-configuration setup in Home Assistant via explicit, retained MQTT discovery payloads published on startup.
3.  **Core System Telemetry:** 60-second updates of CPU Temperature, CPU Usage percentage, RAM percentage and absolute usage (Used & Free in MB), root disk percentage utilization and absolute usage (Used & Free in GB), system load averages (1m, 5m, 15m), and network interface bandwidth (RX/TX kB/s rates).
4.  **Robust Async Lifecycle:** Asynchronous client loop that handles system startup, intermittent network dropouts, and broker reconnects gracefully.
5.  **Service Status Monitoring:** Real-time checking of whitelisted systemd services (e.g., `docker`, `nginx`) via async polling and registers them as binary sensors.
6.  **Layered Configuration:** Supports multi-format configuration files (TOML, YAML, JSON), custom config path overrides via `-c`/`--config` flags, and high-precedence `SYSMQTTD_` prefixed environment variables.
*   **CLI Arguments:** Supports `-h/--help`, `-v/--version`, `-c/--config`, individual parameter override flags (`-H/--host`, `-P/--port`, `-u/--user/--username`, `-w/--password/--pass`, `-p/--prefix`, `-i/--interface`, `-s/--services/--monitored-services`, `-g/--gpio/--gpio-inputs`, and `-o/--gpio-outputs`), and `--verbose` taking absolute highest precedence.
7.  **GPIO Input Monitoring:** Monitors physical state transitions of system GPIO pins configured as inputs (e.g., buttons, magnetic contact door sensors) and publishes changes instantly as Home Assistant binary sensors (`binary_sensor`) with customizable device classes.
8.  **GPIO Output Control:** Actuates physical output devices (like relays, status LEDs, and buzzers) connected to configured GPIO pins via incoming MQTT switch commands, registering them as standard Home Assistant switch entities.
9.  **Safe Remote Commands:** Accepts authorized and whitelisted remote system commands (`reboot`, `shutdown`, `restart_service`) via a secure MQTT subscription, executing them safely and cleanly under non-root privileges.
10. **Native Packaging:** Available as native multi-architecture Debian (`.deb`) and RedHat (`.rpm`) packages for all supported platforms (`x86_64`, `arm64`, `armhf`, `armel`), featuring automatic systemd service provisioning, dedicated non-root execution user setups, and zero-configuration defaults.




