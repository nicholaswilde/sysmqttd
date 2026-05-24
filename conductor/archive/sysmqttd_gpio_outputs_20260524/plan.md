# Implementation Plan: GPIO Output Pin Control for `sysmqttd`

This plan guides the implementation of GPIO output control.

## Phase 1: GPIO Output Core & Subscriptions [checkpoint: 7a32206]
Implement sysfs GPIO output driver and MQTT subscription hooks.

- [x] Task: Implement sysfs-based GPIO output driver
    - [x] Configure direction as `out` and support writing `0` or `1`
- [x] Task: Write unit tests with mock sysfs structure
- [x] Task: Conductor - User Manual Verification 'Phase 1: Output Core' (Protocol in workflow.md)

## Phase 2: Switch Command Integration [checkpoint: 79605e1]
Subscribe to command topics in the event loop and confirm states back.

- [x] Task: Hook subscription commands in eventloop polling
- [x] Task: Add Home Assistant discovery and state feedback topics
- [x] Task: Audit formatting, linting, and coverage gates (>90%)
- [x] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
