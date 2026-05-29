# Implementation Plan: Dynamic Service Control Actuators

This plan guides the implementation of interactive control buttons and switches.

## Phase 1: Command & Handler Expansion [checkpoint: 4f07ac6]
Expand command execution capabilities.

- [x] Task: Expand `RemoteAction` enum to support target service start/stop/restart
- [x] Task: Bind handlers in `daemon.rs` for interactive control topics
- [x] Task: Conductor - User Manual Verification 'Phase 1: Control Handlers'

## Phase 2: discovery registration
Expose buttons and switches to Home Assistant.

- [ ] Task: Setup Auto-Discovery for HA buttons and switches
- [ ] Task: Perform static quality and format checks
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Controls Integration'
