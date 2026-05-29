# Implementation Plan: SD Card Low Disk Space Safe-Guard Protection

This plan guides the implementation of disk space threshold guards and log throttling.

## Phase 1: Capacity Telemetry & Configuration

Expose configurations and update telemetry collection logic.

- [ ] Task: Add `sd_alert_threshold` configuration parsing in `src/config.rs`
- [ ] Task: Update `TelemetryState` and `TelemetryCollector` to calculate and expose `sd_space_alert` state
- [ ] Task: Write TDD unit tests mock capacity triggers
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Telemetry Guard'

## Phase 2: Log Throttling & HA Discovery

Integrate the warning sensor with Home Assistant and implement dynamic logger quiet state overrides.

- [ ] Task: Register the SD Card Space Alert `binary_sensor` discovery payload in `src/discovery.rs` and `src/daemon.rs`
- [ ] Task: Bind a dynamic log filter handler to quiet stdout/stderr logging when alert is active
- [ ] Task: Add unit and integration tests verifying correct auto-discovery and warning state publication
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Log Suppression & HA Warning'
