# Specification: GPIO Input Pin Monitoring for `sysmqttd`

This specification defines the requirements for adding GPIO input monitoring support to `sysmqttd`.

## Overview
Allows monitoring the physical state of system GPIO pins configured as inputs (e.g., buttons, magnetic contact door sensors). The daemon will monitor configured pins and publish state transitions immediately to MQTT.

## Functional Requirements
1. **GPIO Input Scanning**:
   - Leverage `/sys/class/gpio` (via a lightweight native sysfs interface) to read pin values (`0` or `1`).
   - Support edge-trigger interrupt scanning (monitoring both rising and falling edges).
2. **HA Auto-Discovery**:
   - Register monitored inputs as binary sensors (`binary_sensor`) with user-specified device classes (e.g. `door`, `motion`, `window`).
3. **State Publishing**:
   - Immediately publish state changes to `<prefix>/binary_sensor/sysmqttd_<hostname>_pin<pin_number>/state` with `ON` or `OFF`.

## Acceptance Criteria
- Detects physical pin transitions and publishes them instantly.
- Keeps memory and binary size footprint minimal (< 2MB stripped).
- Total code coverage remains >90%.
