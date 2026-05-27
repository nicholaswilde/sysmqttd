# Implementation Plan: Active CPU Usage Percentage Telemetry

This plan guides the implementation of the CPU usage percentage sensor.

## Phase 1: CPU Telemetry Implementation [checkpoint: 8218a90]
Collect CPU usage percentage and add tests.

- [x] Task: Add CPU usage calculation in `telemetry.rs` utilizing `sysinfo::System`
- [x] Task: Update `TelemetryState` to include `cpu_usage` field
- [x] Task: Write unit tests in `telemetry.rs` to verify CPU usage retrieval
- [x] Task: Conductor - User Manual Verification 'Phase 1: CPU Telemetry Core'

## Phase 2: Discovery & Verification [checkpoint: 8db0e8a]
Integrate with discovery and verify.

- [x] Task: Add discovery payload helper in `discovery.rs`
- [x] Task: Register CPU usage discovery config in `daemon.rs`
- [x] Task: Run format and clippy checks (`cargo fmt --all`, `cargo clippy`)
- [x] Task: Conductor - User Manual Verification 'Phase 2: Integration'
