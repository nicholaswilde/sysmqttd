# Specification: GPIO Output Pin Control for `sysmqttd`

This specification defines the requirements for adding GPIO output actuation control support to `sysmqttd`.

## Overview
Allows controlling physical output devices (like relays, status LEDs, and buzzers) connected to system GPIO pins via MQTT switch commands.

## Functional Requirements
1. **GPIO Output Actuation**:
   - Configure whitelisted pins as output and set their state (`0` or `1`) based on commands.
2. **MQTT Command Subscriptions**:
   - Subscribe to command topics: `<prefix>/switch/sysmqttd_<hostname>_pin<pin_number>/set`.
   - Accept payloads `ON` and `OFF`.
3. **HA Auto-Discovery**:
   - Register outputs as switch entities (`switch`) in Home Assistant.

## Acceptance Criteria
- Receiving `ON` or `OFF` on the command topic drives the configured GPIO pin High or Low.
- Confirms state back to state topic.
- Binary size remains lightweight.
