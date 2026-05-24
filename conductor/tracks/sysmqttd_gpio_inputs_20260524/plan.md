# Implementation Plan: GPIO Input Pin Monitoring for `sysmqttd`

This plan guides the implementation of GPIO input monitoring.

## Phase 1: GPIO Sysfs Input Core & Unit Tests
Implement lightweight sysfs-based GPIO input edge listener and verify via unit tests.

- [ ] Task: Implement sysfs-based GPIO input reader
    - [ ] Create `GpioInputListener` using `/sys/class/gpio`
    - [ ] Handle export, direction configuration, and edge listening
- [ ] Task: Write unit tests with a mock sysfs directory structure
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Input Core' (Protocol in workflow.md)

## Phase 2: Async Loop Integration & Discovery
Integrate the GPIO input edge polling task into the main async event loop.

- [ ] Task: Spawn GPIO input polling task in daemon startup
- [ ] Task: Add Home Assistant Auto-Discovery configuration for input sensors
- [ ] Task: Verify formatting, linting, and 90%+ code coverage
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
