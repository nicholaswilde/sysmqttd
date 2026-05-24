# Specification: Multi-Format Configuration File Manager for `sysmqttd`

This specification defines the requirements for expanding `sysmqttd` configuration loading to support multiple file formats, command line arguments, and prefixed environment variables.

## Overview
To increase deployment flexibility, `sysmqttd` will support:
1. Multi-format config files: TOML (`sysmqttd.toml`), YAML (`sysmqttd.yaml`/`sysmqttd.yml`), and JSON (`sysmqttd.json`).
2. Prefixed Environment Variables: All options can be set via environment variables prefixed with `SYSMQTTD_` (e.g., `SYSMQTTD_MQTT_HOST`).
3. CLI Argument Override: Custom config file path specified via `-c` or `--config` flag.

## Functional Requirements
1. **Multi-Format Deserialization**:
   - The daemon must check for and parse the configuration file in three formats:
     - TOML (`sysmqttd.toml` or `/etc/sysmqttd/sysmqttd.toml`)
     - YAML (`sysmqttd.yaml`, `sysmqttd.yml`, or `/etc/sysmqttd/sysmqttd.yaml`)
     - JSON (`sysmqttd.json` or `/etc/sysmqttd/sysmqttd.json`)
2. **Environment Variable Overlay**:
   - Environment variables prefixed with `SYSMQTTD_` must override file configurations.
   - Example mappings:
     - `SYSMQTTD_MQTT_HOST` -> `mqtt_host`
     - `SYSMQTTD_MQTT_PORT` -> `mqtt_port`
     - `SYSMQTTD_MQTT_USER` -> `mqtt_user`
     - `SYSMQTTD_MQTT_PASSWORD` -> `mqtt_password`
     - `SYSMQTTD_MQTT_TOPIC_PREFIX` -> `mqtt_topic_prefix`
3. **Command Line Options**:
   - Command line flag `-c` or `--config <path>` will override the default configuration paths and load the specified config file.
4. **Configuration Precedence (Highest to Lowest)**:
   - Command Line Arguments (for config path)
   - Prefixed Environment Variables (`SYSMQTTD_*`)
   - File Configurations (TOML / YAML / JSON)
   - Built-in Defaults

## Acceptance Criteria
- Running `sysmqttd --config custom_config.json` successfully loads parameters from the custom JSON file.
- Setting `SYSMQTTD_MQTT_HOST=10.0.0.9` overrides the broker host specified in the config file.
- Supports both `.toml`, `.yaml`/`.yml`, and `.json` files.
- Keeps compilation clean, zero memory leaks, and maintains test coverage >90%.
