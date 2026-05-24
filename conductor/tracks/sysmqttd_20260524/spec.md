# Specification - `sysmqttd` Lightweight System Monitoring Daemon

## 1. Overview
`sysmqttd` is a lightweight, native Rust daemon designed to run continuously on low-resource ARMv6 systems (specifically the Raspberry Pi Zero W running DietPi) to collect system telemetry and stream it to a Home Assistant MQTT broker. It supports Home Assistant MQTT Discovery on startup for automatic configuration.

## 2. Target Environment Constraints
*   **Architecture:** `arm-unknown-linux-gnueabihf` (ARMv6 hard-float / ARM1176JZF-S).
*   **Resource Limits:** 512MB RAM, 1GHz single-core CPU. 
    *   **RAM Footprint target:** < 8MB RSS.
    *   **Binary Size target:** < 2MB (stripped release build).
*   **Deployment:** Managed by `systemd` (`sysmqttd.service`) running under a non-root standard user (`sysmqttd` or `dietpi`).

## 3. Tech Stack & Boundaries
*   **Core:** Rust (latest stable).
*   **Crates:**
    *   `sysinfo` (with minimized features: `default-features = false` and only enabling necessary system/disk/cpu components for minimal memory usage).
    *   `rumqttc` (for asynchronous MQTT client loop handling).
    *   `serde` & `serde_json` (for serialization of flat payloads).
*   **Optimization Profile:**
    *   `lto = true` (Link-Time Optimization)
    *   `codegen-units = 1` (Max optimization unit grouping)
    *   `panic = "abort"` (Eliminate unwinding overhead)
    *   `opt-level = "z"` (Optimize for size)
    *   `strip = true` (Remove symbols and debug info)

## 4. Feature Specifications

### 4.1 Configuration
Reads broker credentials from environment variables:
*   `MQTT_HOST` (e.g., `192.168.1.50`, required)
*   `MQTT_PORT` (e.g., `1883`, optional, default: `1883`)
*   `MQTT_USER` (optional)
*   `MQTT_PASSWORD` (optional)
*   `MQTT_TOPIC_PREFIX` (optional, default: `homeassistant`)
Fallback support for a minimal config file `sysmqttd.toml` in `/etc/sysmqttd/` or the local working directory.

### 4.2 Home Assistant MQTT Discovery
On startup, `sysmqttd` must retrieve the local system hostname and publish explicit, **retained** JSON discovery payloads to:
*   `homeassistant/sensor/sysmqttd_<hostname>_cpu_temp/config`
*   `homeassistant/sensor/sysmqttd_<hostname>_ram_usage/config`
*   `homeassistant/sensor/sysmqttd_<hostname>_disk_usage/config`

Each entity must register under a single parent device:
*   **Identifiers:** `["sysmqttd_<hostname>"]`
*   **Name:** `sysmqttd <hostname>`
*   **Model:** `Raspberry Pi Zero W Monitor`
*   **Manufacturer:** `sysmqttd`

#### Example Discovery Payload (CPU Temperature)
```json
{
  "name": "CPU Temperature",
  "stat_t": "homeassistant/sensor/sysmqttd_<hostname>/state",
  "val_tpl": "{{ value_json.cpu_temperature }}",
  "unit_of_meas": "°C",
  "dev_cla": "temperature",
  "state_class": "measurement",
  "uniq_id": "sysmqttd_<hostname>_cpu_temp",
  "dev": {
    "ids": ["sysmqttd_<hostname>"],
    "name": "sysmqttd <hostname>",
    "mdl": "Raspberry Pi Zero W Monitor",
    "mf": "sysmqttd"
  }
}
```

### 4.3 Telemetry Loop
Runs a non-blocking loop every 60 seconds to query:
1.  **CPU Temperature (°C):** Direct thermal sensor read (e.g. via `/sys/class/thermal/thermal_zone0/temp` or minimized `sysinfo` API).
2.  **RAM Usage (%):** Computed as `(used_memory / total_memory) * 100`.
3.  **Disk Storage Utilization (%):** Percentage utilization of the root disk `/` or configured path.

Streams the collected values as a flat JSON state payload to `homeassistant/sensor/sysmqttd_<hostname>/state`:
```json
{
  "cpu_temperature": 43.5,
  "ram_usage": 12.8,
  "disk_usage": 45.2
}
```

## 5. Cross-Compilation & Verification
*   **Toolchain:** `arm-unknown-linux-gnueabihf` target.
*   **Build Utility:** `cross` for containerized environment-free builds on standard dev machines.
*   **Verification Command:** `cross build --target arm-unknown-linux-gnueabihf --release`
