# Implementation Plan: GPIO Output Pin Control for `sysmqttd`

This plan guides the implementation of GPIO output control.

## Phase 1: GPIO Output Core & Subscriptions
Implement sysfs GPIO output driver and MQTT subscription hooks.

- [ ] Task: Implement sysfs-based GPIO output driver
    - [ ] Configure direction as `out` and support writing `0` or `1`
- [ ] Task: Write unit tests with mock sysfs structure
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Output Core' (Protocol in workflow.md)

## Phase 2: Switch Command Integration
Subscribe to command topics in the event loop and confirm states back.

- [ ] Task: Hook subscription commands in eventloop polling
- [ ] Task: Add Home Assistant discovery and state feedback topics
- [ ] Task: Audit formatting, linting, and coverage gates (>90%)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
