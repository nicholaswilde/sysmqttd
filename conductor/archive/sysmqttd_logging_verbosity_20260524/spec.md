# Specification: Logging Verbosity Control for `sysmqttd`

This specification defines the requirements for adding logging verbosity control to the `sysmqttd` daemon.

## Overview
Allows configuring the daemon's log output verbosity. By default, it operates in quiet mode to save SD card write cycles. Enabling verbose mode exposes detailed system runtime data, MQTT event loops, and metrics logs.

## Functional Requirements
1. **Verbosity Flag**:
   - Support a `--verbose` flag (or environment variable `SYSMQTTD_VERBOSE=true`) to toggle verbosity.
2. **Quiet Mode (Default)**:
   - Only log critical startup configurations, connection status, and service errors.
   - Do not log periodic telemetry state payloads or standard GPIO polling runs to standard output.
3. **Verbose Mode**:
   - Log detailed serializations of telemetry metrics payloads before publication.
   - Log exact incoming and outgoing MQTT packets (ConnAck, Publish, etc.) inside the daemon async event loop.
   - Log initial states and state transitions for monitored systemd services and GPIO inputs.

## Acceptance Criteria
- Quiet by default; no output for 60-second telemetry publications.
- Emits detailed debug prints only when `--verbose` or `SYSMQTTD_VERBOSE=true` is enabled.
- Compiles cleanly on arm-unknown-linux-gnueabihf target.
