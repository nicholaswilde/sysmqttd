# Specification: Safe Remote Commands for `sysmqttd`

This specification defines the requirements for implementing whitelisted remote commands via MQTT.

## Overview
Allows authorized operators to remotely command safe operations like `reboot`, `shutdown`, or `restart_service` using whitelisted inputs on an MQTT topic.

## Functional Requirements
1. **Command Subscriptions**:
   - Subscribe to `<prefix>/sensor/sysmqttd_<hostname>/command`.
2. **Safe Action Whitelist**:
   - Accept only `reboot`, `shutdown`, or `restart_service`.
   - Ignore any other string inputs or parameters.
3. **Execution**:
   - Boot under non-root permissions, executing whitelisted controls cleanly.

## Acceptance Criteria
- Subscribes to commands topic.
- Successfully parses and validates input.
- High security and zero arbitrary execution.
