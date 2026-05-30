# Implementation Plan: SD Card Low Disk Space Safe-Guard Protection

This plan guides the implementation of disk space threshold guards and log throttling.

## Phase 1: Capacity Telemetry & Configuration [checkpoint: 2bb1823]

Expose configurations and update telemetry collection logic.

- [x] Task: Add `sd_alert_threshold` configuration parsing in `src/config.rs`
- [x] Task: Update `TelemetryState` and `TelemetryCollector` to calculate and expose `sd_space_alert` state
- [x] Task: Write TDD unit tests mock capacity triggers
- [x] Task: Conductor - User Manual Verification 'Phase 1: Telemetry Guard'

## Phase 2: Log Throttling & HA Discovery [checkpoint: 1f189ce]

Integrate the warning sensor with Home Assistant and implement dynamic logger quiet state overrides.

- [x] Task: Register the SD Card Space Alert `binary_sensor` discovery payload in `src/discovery.rs` and `src/daemon.rs`
- [x] Task: Bind a dynamic log filter handler to quiet stdout/stderr logging when alert is active
- [x] Task: Add unit and integration tests verifying correct auto-discovery and warning state publication
- [x] Task: Conductor - User Manual Verification 'Phase 2: Log Suppression & HA Warning'
