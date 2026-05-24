# Implementation Plan: Systemd Service Status Binary Monitor for `sysmqttd`

This plan guides the implementation of the systemd service status binary monitor feature.

## Phase 1: Status Checking Core & Unit Tests
Implement a lightweight systemd active service status reader and verify via unit tests.

- [ ] Task: Implement lightweight systemd service status reader
- [ ] Task: Write unit tests verifying state classification
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Core Checking' (Protocol in workflow.md)

## Phase 2: Discovery and State Integration
Integrate the binary sensors inside the Home Assistant discovery payload and state loop.

- [ ] Task: Integrate binary sensors into Home Assistant discovery and state loops
- [ ] Task: Run coverage audits to ensure >90% coverage
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
