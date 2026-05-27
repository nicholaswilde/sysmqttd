# Implementation Plan: SBC Hardware Health Diagnostics

This plan guides the implementation of hardware power and thermal warning sensors.

## Phase 1: Kernel Interface & Mock Tests
Implement sysfs parsing.

- [ ] Task: Implement robust sysfs parser in `telemetry.rs` for under-voltage and throttle flags
- [ ] Task: Update `TelemetryState` to include `undervoltage_detected` and `throttled`
- [ ] Task: Add test suite using mock system layout
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Hardware Health Core'

## Phase 2: Discovery Configuration
Add discovery configuration and run static validation.

- [ ] Task: Implement auto-discovery for both binary sensors in `discovery.rs` and `daemon.rs`
- [ ] Task: Run clippy and cross-compile checks
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Discovery Integration'
