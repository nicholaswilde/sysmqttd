# Implementation Plan: Pending System Updates and Top Process Diagnostics

This plan guides the implementation of package updates and top process tracking.

## Phase 1: Aggregation Logic
Fetch package counts and top process data.

- [ ] Task: Implement daily slow-loop check for system updates in `telemetry.rs`
- [ ] Task: Implement process list parser in `telemetry.rs` for top resource consumers
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Diagnostics Core'

## Phase 2: Home Assistant Exposure
Add discovery and verify.

- [ ] Task: Create discovery definitions in `discovery.rs`
- [ ] Task: Bind to discovery cycle in `daemon.rs`
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Diagnostics Integration'
