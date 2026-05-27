# Implementation Plan: Absolute RAM and Disk Usage Telemetry

This plan guides the implementation of raw RAM and disk capacity metrics.

## Phase 1: Core Parsing & Testing
Expose raw telemetry fields.

- [ ] Task: Update telemetry collector in `telemetry.rs` to extract free/used RAM (MB) and Disk (GB)
- [ ] Task: Update `TelemetryState` struct with absolute fields
- [ ] Task: Write unit tests in `telemetry.rs` to verify correct conversions
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Absolute Telemetry Core'

## Phase 2: Discovery & Validation
Integrate with discovery and verify.

- [ ] Task: Implement discovery configuration helpers in `discovery.rs`
- [ ] Task: Publish config payloads in `daemon.rs`
- [ ] Task: Verify that all standard tests and code linters pass successfully
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Discovery Integration'
