# Product: `sysmqttd` System Monitor

## Vision
A highly-efficient, lightweight system monitoring daemon for resource-constrained ARMv6 single-board computers (specifically Raspberry Pi Zero W running DietPi). It seamlessly integrates with Home Assistant via MQTT Discovery and provides real-time system metrics (CPU Temp, RAM usage, Disk usage) with a negligible resource footprint.

## Target Audience
System administrators, makers, and smart home enthusiasts running Home Assistant who want to monitor their low-power single-board computers without incurring high memory or CPU overhead.

## Key Features
1.  **Tiny Footprint:** Native compiled Rust binary optimized for size (< 2MB stripped) and memory (< 8MB RAM).
2.  **Home Assistant Autodiscovery:** Zero-configuration setup in Home Assistant via explicit, retained MQTT discovery payloads published on startup.
3.  **Core System Telemetry:** 60-second updates of CPU Temperature, RAM percentage, root disk percentage utilization, system load averages (1m, 5m, 15m), and network interface bandwidth (RX/TX kB/s rates).
4.  **Robust Async Lifecycle:** Asynchronous client loop that handles system startup, intermittent network dropouts, and broker reconnects gracefully.
5.  **Service Status Monitoring:** Real-time checking of whitelisted systemd services (e.g., `docker`, `nginx`) via async polling and registers them as binary sensors.
6.  **Layered Configuration:** Supports multi-format configuration files (TOML, YAML, JSON), custom config path overrides via `-c`/`--config` flags, and high-precedence `SYSMQTTD_` prefixed environment variables.
*   **CLI Arguments:** `-h/--help`, `-v/--version`, and `-c/--config` flags for configuration, usage, and version information.


