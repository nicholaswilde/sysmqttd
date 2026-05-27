# Implementation Plan: Active CPU Usage Percentage Telemetry

This plan guides the implementation of the CPU usage percentage sensor.

## Phase 1: CPU Telemetry Implementation
Collect CPU usage percentage and add tests.

- [ ] Task: Add CPU usage calculation in `telemetry.rs` utilizing `sysinfo::System`
- [ ] Task: Update `TelemetryState` to include `cpu_usage` field
- [ ] Task: Write unit tests in `telemetry.rs` to verify CPU usage retrieval
- [ ] Task: Conductor - User Manual Verification 'Phase 1: CPU Telemetry Core'

## Phase 2: Discovery & Verification
Integrate with discovery and verify.

- [ ] Task: Add discovery payload helper in `discovery.rs`
- [ ] Task: Register CPU usage discovery config in `daemon.rs`
- [ ] Task: Run format and clippy checks (`cargo fmt --all`, `cargo clippy`)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration'
