# Specification: Absolute RAM and Disk Usage Telemetry

This specification defines the requirements for collecting and publishing absolute RAM and disk storage utilization.

## Overview
Expose raw system capacity metrics (e.g. Free/Used Memory in MB, Free/Used Disk space in GB) alongside the percentage-based metrics currently collected.

## Functional Requirements
1. **Absolute Telemetry Collection**:
   - Collect RAM used and free values in MB (megabytes).
   - Collect root disk used and free values in GB (gigabytes).
2. **State Payload Integration**:
   - Fields: `"ram_used_mb"`, `"ram_free_mb"`, `"disk_used_gb"`, `"disk_free_gb"`.
3. **HA Auto-Discovery**:
   - Register sensors with Home Assistant using appropriate unit of measurements (`MB` and `GB`) and `measurement` state class.

## Acceptance Criteria
- Telemetry payload includes raw size values.
- Home Assistant Auto-Discovery configuration payloads are published on startup.
- Telemetry calculations are resource efficient.
