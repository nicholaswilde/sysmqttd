# Specification: Systemd Service Status Binary Monitor for `sysmqttd`

This specification defines the requirements for adding a systemd service status binary monitor to `sysmqttd`.

## Overview
This feature allows monitoring the active status of critical services (e.g., `docker`, `nginx`) via binary sensors in Home Assistant.

## Functional Requirements
1. **Service Status Check**:
   - Read systemd service states by checking the sysfs cgroup active status or querying systemd dbus properties, or parsing `/sys/fs/cgroup/system.slice/...` (maintaining zero-dependency lightweight bounds).
   - Alternatively, monitor status by reading `/etc/systemd/system` configurations or executing a lightweight internal check.
2. **HA Auto-Discovery**:
   - Expose binary sensors (`binary_sensor`) with `connectivity` or `problem` classes representing `online`/`offline`.
3. **Payload Stream**:
   - Stream service active status binary keys.

## Acceptance Criteria
- Selected systemd service states are exposed.
- Zero-dependency design is retained.
