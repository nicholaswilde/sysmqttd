# Specification: SD Card Low Disk Space Safe-Guard Protection

This specification defines the requirements for adding a low-disk safe-guard to `sysmqttd` to prevent filesystem lockups and protect SD card lifetime.

## Overview
Raspberry Pi SD cards running local filesystems are prone to severe corruption and lockups when root disk capacity reaches 100% or under heavy journal writing. This feature throttles logging and alerts the user when disk space is critically low.

## Functional Requirements
1. **Critical Capacity Threshold Monitoring:**
   - Periodically monitor the active root `/` directory utilization percentage (via the `TelemetryCollector`'s existing disk reading cycle).
   - Define a customizable trigger threshold (defaulting to `95%` utilization).
2. **Dynamic Alert binary_sensor:**
   - Expose a new Home Assistant binary sensor `"SD Card Space Alert"` (`binary_sensor`) with `problem` device class.
   - Set state to `"ON"` when disk utilization exceeds the configured threshold.
3. **Aggressive Logging Throttling:**
   - When the threshold is exceeded, the daemon must instantly toggle its active logging level to quiet/off, discarding all debug/verbose logs that write to standard streams to prevent journald writing loops on a full SD card.

## Acceptance Criteria
- Telemetry payload includes `"sd_space_alert"` (boolean).
- Discovery configuration correctly registers the alert sensor under `problem` device class.
- Verified mock capacity tests successfully trigger quiet logging mode.
