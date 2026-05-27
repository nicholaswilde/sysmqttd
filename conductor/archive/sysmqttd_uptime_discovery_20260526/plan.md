# Implementation Plan: System Uptime Discovery Registration

This plan guides the integration of the system uptime sensor discovery registration.

## Phase 1: Implementation
Register the sensor and add tests.

- [ ] Task: Implement `new_uptime` discovery payload helper in `discovery.rs`
- [ ] Task: Publish uptime discovery payload on connection in `daemon.rs`
- [ ] Task: Add unit tests in `discovery.rs` to verify correct serialization
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Uptime Discovery'

## Phase 2: Integration & Verification
Verify all checks.

- [ ] Task: Run format and clippy checks (`cargo fmt --all`, `cargo clippy`)
- [ ] Task: Verify that unit tests pass successfully
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration & Formatting'
