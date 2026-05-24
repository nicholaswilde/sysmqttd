# Specification: Command-Line Argument Equivalents for All Environment Variables

This specification defines the requirements for adding CLI argument equivalents for all environment variables in `sysmqttd`.

## Overview
To provide a first-class command-line interface and increase deployment flexibility, `sysmqttd` will support overriding all configuration options via command-line arguments. Each configuration environment variable will have an equivalent long and short command-line flag.

## Functional Requirements
1. **CLI Parameter Mapping**:
   The following command-line flags must map directly to their corresponding configuration parameters:
   - `-H`, `--host <host>` -> `mqtt_host`
   - `-P`, `--port <port>` -> `mqtt_port`
   - `-u`, `--user <username>` -> `mqtt_user`
   - `-w`, `--password <password>` -> `mqtt_password`
   - `-p`, `--prefix <prefix>` -> `mqtt_topic_prefix`
   - `-i`, `--interface <interface>` -> `net_interface`
   - `-s`, `--services <services>` -> `MONITORED_SERVICES`

2. **Precedence Hierarchy**:
   Command-line parameter arguments must take absolute highest precedence, overriding all environment variables, configuration files, and defaults:
   1. Command-Line Arguments (both custom config path and individual parameter flags)
   2. Prefixed Environment Variables (`SYSMQTTD_*`)
   3. Legacy Environment Variables (`MQTT_*`, `NET_INTERFACE`, `MONITORED_SERVICES`)
   4. Configuration File (TOML, YAML, JSON)
   5. Built-in Defaults

3. **CLI Usage & Help Documentation**:
   - The usage screen (`-h` / `--help`) must be updated to clearly document all new CLI options, their parameter types, and their equivalent environment variables.

## Acceptance Criteria
- Running `sysmqttd -H 10.0.0.10 -P 1884` successfully overrides any broker host and port settings in files and env vars.
- All new command-line flags are properly parsed and handled.
- Keeps compilation clean, zero memory leaks, and maintains test coverage >90%.
