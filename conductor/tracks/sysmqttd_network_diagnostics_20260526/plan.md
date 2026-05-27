# Implementation Plan: Network Diagnostics and Wi-Fi RSSI

This plan guides the implementation of interface addresses and signal telemetry.

## Phase 1: Query API & Parse Proc
Retrieve network info.

- [ ] Task: Implement interface IP/MAC discovery helper in `telemetry.rs`
- [ ] Task: Implement `/proc/net/wireless` RSSI parser
- [ ] Task: Update `TelemetryState` struct
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Network Core'

## Phase 2: Discovery Integration
Expose to Home Assistant.

- [ ] Task: Add discovery payloads for the diagnostic network sensors
- [ ] Task: Test and formatting validation
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Network Discovery'
