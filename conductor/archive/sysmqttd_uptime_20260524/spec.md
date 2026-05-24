# Specification: System Uptime Sensor for `sysmqttd`

This specification defines the requirements for adding a system uptime sensor to `sysmqttd`.

## Overview
To monitor system stability and uptime duration, the daemon will periodically parse `/proc/uptime`, format the total seconds as an uptime metric, and publish it via MQTT with Home Assistant Auto-Discovery support.

## Functional Requirements
1. **Uptime Metric Extraction**:
   - The daemon must read `/proc/uptime` at each polling interval.
   - The first float value in `/proc/uptime` represents the total seconds the system has been up.
2. **HA Auto-Discovery**:
   - Topic: `<prefix>/sensor/sysmqttd_<hostname>_uptime/config`
   - Expose as a sensor with `duration` device class and `s` (seconds) unit of measurement.
3. **Payload Stream**:
   - The computed uptime seconds must be added to the unified telemetry state JSON payload:
     `"uptime": <seconds>`

## Acceptance Criteria
- Running `sysmqttd` includes the `"uptime"` field in the state payload.
- Home Assistant Auto-Discovery configuration payload is published on connection.
- Total line coverage remains above 90% with zero memory leaks.
