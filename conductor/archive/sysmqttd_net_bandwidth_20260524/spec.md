# Specification: Network Interface Bandwidth for `sysmqttd`

This specification defines the requirements for adding RX/TX network bandwidth rate sensors to `sysmqttd`.

## Overview
Monitoring network throughput is essential for remote Pi nodes. The daemon will periodically read cumulative RX/TX bytes from `/proc/net/dev` for a user-specified interface (defaults to `wlan0`), calculate transfer rates (kB/s) between polling intervals, and export them to MQTT.

## Functional Requirements
1. **Bandwidth Computation**:
   - Extract cumulative RX and TX bytes for a configured network interface (e.g. `wlan0` or `eth0`) from `/proc/net/dev`.
   - Track previous read times and values to calculate rates: `rate = (current_bytes - previous_bytes) / delta_seconds`.
2. **HA Auto-Discovery**:
   - Register `Network RX Rate` and `Network TX Rate` sensors with `kB/s` unit.
3. **Payload Stream**:
   - Add `"net_rx_rate"` and `"net_tx_rate"` keys to the state payload.

## Acceptance Criteria
- Bandwidth rates are computed and streamed correctly.
- Test coverage maintains 90%+ target.
