# Implementation Plan: CPU Load Averages for `sysmqttd`

This plan guides the implementation of the CPU load averages feature.

## Phase 1: Load Avg Core & Tests
Implement load average metrics extraction and verify via unit tests.

- [ ] Task: Parse /proc/loadavg to extract averages
    - [ ] Create `read_load_avg() -> Result<(f64, f64, f64), String>` in `telemetry.rs`
- [ ] Task: Write unit tests with mock loadavg file
    - [ ] Test successful parsing of load averages
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Load Avg Core' (Protocol in workflow.md)

## Phase 2: Integration & Audits
Integrate load averages into state and discovery payloads.

- [ ] Task: Integrate load metrics in state and discovery
    - [ ] Update `TelemetryState` and `DiscoveryPayload`
    - [ ] Verify coverage exceeds 90% (`task coverage`)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
