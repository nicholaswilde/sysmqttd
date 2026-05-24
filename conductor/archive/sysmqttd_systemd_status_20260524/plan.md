# Implementation Plan: Systemd Service Status Binary Monitor for `sysmqttd`

This plan guides the implementation of the systemd service status binary monitor feature.

## Phase 1: Status Checking Core & Unit Tests [checkpoint: 2fa3189]
Implement a lightweight systemd active service status reader and verify via unit tests.

- [x] Task: Implement lightweight systemd service status reader
- [x] Task: Write unit tests verifying state classification
- [x] Task: Conductor - User Manual Verification 'Phase 1: Core Checking' (Protocol in workflow.md)

## Phase 2: Discovery and State Integration [checkpoint: 2fa3189]
Integrate the binary sensors inside the Home Assistant discovery payload and state loop.

- [x] Task: Integrate binary sensors into Home Assistant discovery and state loops
- [x] Task: Run coverage audits to ensure >90% coverage
- [x] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
