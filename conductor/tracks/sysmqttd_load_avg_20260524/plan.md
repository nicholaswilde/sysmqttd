# Implementation Plan: CPU Load Averages for `sysmqttd`

This plan guides the implementation of the CPU load averages feature.

## Phase 1: Load Avg Core & Tests [checkpoint: c2eee5f]
Implement load average metrics extraction and verify via unit tests.

- [x] Task: Parse /proc/loadavg to extract averages
    - [x] Create `read_load_avg() -> Result<(f64, f64, f64), String>` in `telemetry.rs`
- [x] Task: Write unit tests with mock loadavg file
    - [x] Test successful parsing of load averages
- [x] Task: Conductor - User Manual Verification 'Phase 1: Load Avg Core' (Protocol in workflow.md)

## Phase 2: Integration & Audits [checkpoint: c2eee5f]
Integrate load averages into state and discovery payloads.

- [x] Task: Integrate load metrics in state and discovery
    - [x] Update `TelemetryState` and `DiscoveryPayload`
    - [x] Verify coverage exceeds 90% (`task coverage`)
- [x] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
