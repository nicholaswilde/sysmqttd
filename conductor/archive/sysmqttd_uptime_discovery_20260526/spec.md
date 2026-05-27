# Specification: System Uptime Discovery Registration

This specification defines the requirements for registering the system uptime sensor with Home Assistant MQTT Discovery.

## Overview
Although `uptime_seconds` is currently parsed and published in the JSON telemetry state payload, it is not registered via Home Assistant Auto-Discovery. This track registers the entity so it appears automatically in Home Assistant.

## Functional Requirements
1. **HA Auto-Discovery Registration**:
   - Topic: `<prefix>/sensor/sysmqttd_<hostname>_uptime/config`
   - Payload must define the sensor with `duration` device class and `s` (seconds) unit of measurement.
   - Value template: `{{ value_json.uptime_seconds }}`
2. **Quality Verification**:
   - Ensure the entity correctly registers and reports values under the parent `sysmqttd` device.

## Acceptance Criteria
- Home Assistant Auto-Discovery configuration payload is published for `uptime` on startup.
- Uptime sensor correctly shows up as a duration in Home Assistant.
- Unit tests verify the discovery payload structure.
