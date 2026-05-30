# Specification: Structured CLI Healthcheck Command

This specification defines the requirements for adding a dedicated `--healthcheck` mode to the `sysmqttd` daemon.

## Overview
Enables systemd service managers and container runtimes to programmatically evaluate if the daemon is functional, properly configured, able to gather telemetry, and capable of reaching the MQTT broker.

## Functional Requirements
1. **CLI Trigger:**
   - Expose `-k` / `--healthcheck` boolean CLI flag.
2. **Ephemeral Diagnostics Lifecycle:**
   - When triggered, `sysmqttd` must execute in a short-lived diagnostic mode (it does not start the long-running daemon loop).
   - Load the active TOML/YAML/JSON configuration.
   - Run a single-cycle telemetry gather command on the configured network interface to ensure permissions and `/proc`/`/sys` readers are fully functional.
   - Initiate a transient, connection-only handshake with the MQTT broker (connecting, checking session state, and cleanly disconnecting).
3. **Exit Code Semantics:**
   - Exit with status `0` if all diagnostic checks pass cleanly.
   - Exit with non-zero status codes on failure (e.g., `1` for config errors, `2` for local telemetry gather failures, `3` for broker connection timeouts/refusals).

## Acceptance Criteria
- Running `sysmqttd --healthcheck` finishes within 2-3 seconds.
- Normal daemon execution remains fully unaffected.
- Unit/integration tests verify correct exit codes under simulated failures.
