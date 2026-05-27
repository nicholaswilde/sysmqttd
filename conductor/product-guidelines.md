# Product Guidelines: `sysmqttd`

This document defines the guidelines and standards for the `sysmqttd` daemon's telemetry, logging, and naming conventions.

## 1. Naming Conventions
*   **MQTT Client ID:** Must follow the pattern `sysmqttd_<hostname>` to ensure uniqueness across the broker network.
*   **Home Assistant Unique IDs:** All discovery entities must use `sysmqttd_<hostname>_<metric>` (e.g., `sysmqttd_pi-zero_cpu_temp`).
*   **Home Assistant Device Identifier:** `sysmqttd_<hostname>` is the parent identifier. All sensors must list this identifier in their `device.identifiers` list.
*   **Remote Command Topic:** `<prefix>/sensor/sysmqttd_<hostname>/command` (used for receiving whitelisted safe commands: `reboot`, `shutdown`, `restart_service`).

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
*   **Verbosity Control:**
    *   **Quiet Mode (Default):** Operates silently (no logging of periodic 60-second telemetry publishing payloads, standard GPIO polling runs, systemd service checks, or MQTT event loop packets) to protect single-board computer SD cards from excessive write cycles.
    *   **Verbose Mode:** Enabled via `--verbose` CLI parameter or `SYSMQTTD_VERBOSE=true` environment variable. In this mode, the daemon logs detailed telemetry payloads before publication, exact incoming and outgoing MQTT packets (such as ConnAck, Publish, etc.) inside the async event loop, and all initial states/transitions for systemd services and GPIO inputs.

## 4. Documentation Standards
*   **README.md and sysmqttd.toml.example Updates**: The project `README.md` and `sysmqttd.toml.example` at the repository root must be updated each time a feature is added, changed, or refined. This guarantees that deployment guides, CLI usage flags, default values, configuration references, and example files are always accurate and synchronised with the executable.

