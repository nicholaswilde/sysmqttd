# Specification: Dynamic Service Control Actuators

This specification defines the requirements for implementing remote button actuators and service controllers.

## Overview
Allows clean startup/shutdown/restart of systemd services or host systems via Home Assistant button or switch controls.

## Functional Requirements
1. **Interactive Entities**:
   - Register Home Assistant `button` entities for Reboot/Shutdown.
   - For monitored systemd services, expose dynamic control switches allowing start/stop/restart.
2. **Subscribed Commands**:
   - Subscribe to action topics such as `<prefix>/switch/sysmqttd_<hostname>_service_<name>/set`.
   - Safely process action commands (`ON` / `OFF` / `RESTART`).

## Acceptance Criteria
- Interactive control elements appear in Home Assistant.
- Verification checks prevent arbitrary shell command injection.
