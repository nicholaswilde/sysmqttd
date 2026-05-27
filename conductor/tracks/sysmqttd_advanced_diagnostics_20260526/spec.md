# Specification: Pending System Updates and Top Process Diagnostics

This specification defines the requirements for reporting pending package updates and active top resource consumers.

## Overview
Adds administrative telemetry including apt upgradable package count and top resource consuming processes.

## Functional Requirements
1. **Package Updates Check**:
   - Check pending system package upgrades (e.g., parsing `/var/lib/apt/lists` or run lightweight check).
2. **Top Consumer Process**:
   - Traverse active process list to identify the top CPU or RAM consumer.
3. **HA Auto-Discovery**:
   - Expose upgradable package count as an `update` sensor.
   - Expose top process details as a diagnostic string sensor.

## Acceptance Criteria
- Sensors successfully populate in HA.
- Update check executes on a separate slow loop (e.g., daily) to prevent system overhead.
