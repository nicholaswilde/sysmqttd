# Specification: SBC Hardware Health Diagnostics

This specification defines the requirements for monitoring SBC hardware power and throttling health states.

## Overview
Raspberry Pi and other SBCs are highly sensitive to power delivery and overheating. This track reads kernel interfaces to publish binary sensors for active under-voltage and thermal throttling.

## Functional Requirements
1. **Health State Monitoring**:
   - Parse `/sys/devices/platform/soc/soc:firmware/get_throttled` or `/sys/class/power_supply` to extract system power/thermal flags (or fallbacks for non-Pi Linux systems).
2. **State Payload Integration**:
   - Include `"undervoltage_detected"` (bool) and `"throttled"` (bool) in telemetry.
3. **HA Auto-Discovery**:
   - Register both as binary sensors with device class `problem`.

## Acceptance Criteria
- Telemetry payload includes boolean flags for both diagnostics.
- Auto-discovery registers both binary sensors under the parent device.
- Robust fallback logic exists for non-SBC host platforms.
