# Product Guidelines: `sysmqttd`

This document defines the guidelines and standards for the `sysmqttd` daemon's telemetry, logging, and naming conventions.

## 1. Naming Conventions
*   **MQTT Client ID:** Must follow the pattern `sysmqttd_<hostname>` to ensure uniqueness across the broker network.
*   **Home Assistant Unique IDs:** All discovery entities must use `sysmqttd_<hostname>_<metric>` (e.g., `sysmqttd_pi-zero_cpu_temp`).
*   **Home Assistant Device Identifier:** `sysmqttd_<hostname>` is the parent identifier. All sensors must list this identifier in their `device.identifiers` list.

## 2. Telemetry and Payload Conventions
*   **Discovery Payloads:**
    *   Must be published with the `retain` flag set to `true`.
    *   Topic format: `<prefix>/sensor/sysmqttd_<hostname>_<metric>/config` (where `<prefix>` is configured, defaulting to `homeassistant`).
*   **State Payloads:**
    *   Must be published with the `retain` flag set to `false` (to avoid outdated state on restart).
    *   Topic format: `<prefix>/sensor/sysmqttd_<hostname>/state`.
    *   Payload format: Flat JSON object containing only numeric values.
*   **Numerical Precision:**
    *   CPU Temperature: Float rounded to 1 decimal place (e.g., `45.2`).
    *   RAM Usage: Float rounded to 1 decimal place (e.g., `12.5`).
    *   Disk Usage: Float rounded to 1 decimal place (e.g., `32.1`).

## 3. Logging & Diagnostics
*   Logs must be written to standard output (`stdout`/`stderr`) using standard Rust logging crates (e.g., `env_logger` or `tracing` configured for simple outputs) so that systemd can capture them in the journal.
*   Log levels:
    *   `INFO`: Startup announcements, successful broker connections, successful HA discovery registration.
*   `WARN`: Non-fatal issues (e.g., failed to read disk stats, temporary MQTT reconnect attempts).
    *   `ERROR`: Fatal issues (e.g., failed to load configuration, broker connection completely lost after max retries).
*   **Silent operation:** Telemetry loops must not log on every 60-second publish to avoid filling the systemd journal on low-resource flash media.

## 4. Documentation Standards
*   **README.md Updates**: The project `README.md` at the repository root must be updated each time a feature is added, changed, or refined. This guarantees that deployment guides, CLI usage flags, and configuration examples are always accurate and synchronised with the executable.

