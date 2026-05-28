# Specification: Network Diagnostics and Wi-Fi RSSI

This specification defines the requirements for reporting host IP/MAC addresses and Wi-Fi RSSI signal strength.

## Overview
Enables debugging wireless dropouts and dynamic IP addressing issues.

## Functional Requirements
1. **Network Info Retrieval**:
   - Query interface properties to fetch active IP and MAC address of the monitored NIC.
   - For wireless NICs, parse `/proc/net/wireless` to retrieve link quality or RSSI.
2. **State Payload Integration**:
   - Include `"ip_address"`, `"mac_address"`, and `"wifi_rssi"` (if applicable).
3. **HA Auto-Discovery**:
   - Register IP/MAC as diagnostic sensors, and Wi-Fi RSSI using `signal_strength` device class.

## Acceptance Criteria
- Diagnostic sensors correctly populate in Home Assistant.
- Interface bandwidth calculations remain unaffected by IP lookup calls.
