# Implementation Plan: System Uptime Sensor for `sysmqttd`

This plan guides the implementation of the system uptime sensor feature.

## Phase 1: Uptime Reading & Unit Tests
Implement the logic to extract uptime and verify it with unit tests.

- [ ] Task: Parse /proc/uptime to extract system uptime
    - [ ] Create `read_uptime() -> Result<f64, String>` in `telemetry.rs`
- [ ] Task: Write unit tests with a mock uptime file root
    - [ ] Test successful parsing of uptime float
    - [ ] Test error fallback
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Uptime Core' (Protocol in workflow.md)

## Phase 2: Integration & Verification
Integrate the uptime metrics in telemetry and verify it passes quality metrics.

- [ ] Task: Integrate uptime metric in telemetry and discovery payloads
    - [ ] Update `TelemetryState` and `DiscoveryPayload` structures
    - [ ] Run clippy and format checks (`task lint` / `task fmt`)
    - [ ] Verify coverage remains above 90% (`task coverage`)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
