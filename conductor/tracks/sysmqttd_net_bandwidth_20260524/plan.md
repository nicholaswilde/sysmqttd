# Implementation Plan: Network Interface Bandwidth for `sysmqttd`

This plan guides the implementation of the network interface bandwidth tracking feature.

## Phase 1: Bandwidth Parsing Core
Implement /proc/net/dev parser and verify it with unit tests.

- [ ] Task: Extract interface metrics from /proc/net/dev
    - [ ] Add `read_interface_bytes(interface: &str) -> Result<(u64, u64), String>` in `telemetry.rs`
- [ ] Task: Write unit tests validating parsing of /proc/net/dev structure
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Parsing Core' (Protocol in workflow.md)

## Phase 2: Bandwidth Tracking & Integration
Add tracking state and integrate metrics into telemetry loop.

- [ ] Task: Keep track of previous rates and integrate metrics into telemetry loop
    - [ ] Calculate actual kB/s delta rates inside telemetry loop
    - [ ] Add `net_rx_rate` and `net_tx_rate` to configuration, state, and discovery
    - [ ] Verify formatting, linting, and coverage gates (`task coverage`)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
