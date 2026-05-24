# Specification: CPU Load Averages for `sysmqttd`

This specification defines the requirements for adding CPU load average sensors (1m, 5m, 15m) to `sysmqttd`.

## Overview
CPU load averages provide a long-term metric of system load. The daemon will read `/proc/loadavg` periodically and publish load average metrics to MQTT.

## Functional Requirements
1. **Load Avg Parsing**:
   - The daemon must read `/proc/loadavg` at each polling interval.
   - Extract the first three floats representing 1-minute, 5-minute, and 15-minute load averages.
2. **HA Auto-Discovery**:
   - Register three entities: `Load Avg (1m)`, `Load Avg (5m)`, and `Load Avg (15m)`.
3. **Payload Stream**:
   - Add `"load_1m"`, `"load_5m"`, and `"load_15m"` to the unified telemetry state payload.

## Acceptance Criteria
- Telemetry payload includes three load averages.
- Home Assistant Auto-Discovery is fully configured.
- Line coverage target (>90%) is met.
