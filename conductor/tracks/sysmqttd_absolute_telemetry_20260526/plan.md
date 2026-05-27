# Implementation Plan: Absolute RAM and Disk Usage Telemetry

This plan guides the implementation of raw RAM and disk capacity metrics.

## Phase 1: Core Parsing & Testing [checkpoint: c08bd57]
Expose raw telemetry fields.

- [x] Task: Update telemetry collector in `telemetry.rs` to extract free/used RAM (MB) and Disk (GB)
- [x] Task: Update `TelemetryState` struct with absolute fields
- [x] Task: Write unit tests in `telemetry.rs` to verify correct conversions
- [x] Task: Conductor - User Manual Verification 'Phase 1: Absolute Telemetry Core'

## Phase 2: Discovery & Validation
Integrate with discovery and verify.

- [x] Task: Implement discovery configuration helpers in `discovery.rs`
- [x] Task: Publish config payloads in `daemon.rs`
- [x] Task: Verify that all standard tests and code linters pass successfully
- [x] Task: Conductor - User Manual Verification 'Phase 2: Discovery Integration'
