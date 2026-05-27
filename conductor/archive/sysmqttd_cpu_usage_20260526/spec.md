# Specification: Active CPU Usage Percentage Telemetry

This specification defines the requirements for collecting and publishing overall CPU usage percentage.

## Overview
Extend telemetry collection to include active CPU usage percentage (from 0% to 100%) and register it as a Home Assistant sensor.

## Functional Requirements
1. **CPU Usage Telemetry Collection**:
   - The daemon must calculate system CPU usage during the polling period.
   - Utilize minimized `sysinfo` features (e.g., `sys.refresh_cpu_usage()`).
2. **State Payload Integration**:
   - Topic: `<prefix>/sensor/sysmqttd_<hostname>/state`
   - Include `"cpu_usage"` field in the JSON telemetry state payload.
3. **HA Auto-Discovery**:
   - Topic: `<prefix>/sensor/sysmqttd_<hostname>_cpu_usage/config`
   - Register sensor with state class `measurement`, unit `%`, and no device class.

## Acceptance Criteria
- State payload includes the `"cpu_usage"` field.
- Home Assistant Auto-Discovery configuration payload is published on connection.
- Total line coverage remains above 90% with minimal CPU and memory overhead.
