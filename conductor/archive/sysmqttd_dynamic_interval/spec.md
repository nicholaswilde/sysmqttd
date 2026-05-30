# Specification: Dynamic Polling Interval Adjustment via MQTT Command

This specification defines the requirements for dynamically adjusting the telemetry polling interval of the `sysmqttd` daemon via incoming MQTT command payloads.

## Overview
Currently, the daemon polling interval is fixed at startup based on the configuration file. To enable real-time control, the daemon should listen to a specific MQTT topic for interval changes, validate the inputs, dynamically update its polling timer loop in-memory, and report the new interval back to the broker.

## Functional Requirements
1. **MQTT Command Subscription:**
   - Subscribe to the command topic: `<prefix>/sensor/sysmqttd_<hostname>/interval/set`.
2. **Payload Parsing & Validation:**
   - Parse incoming payloads as integers (representing polling interval in seconds).
   - Enforce a minimum bound of `1` second (to prevent high-frequency loop spam) and a maximum bound of `86400` seconds (24 hours).
   - Reject invalid non-numeric/empty payloads with an error log, maintaining the existing interval.
3. **Dynamic Loop Override:**
   - Dynamically update the tokio time interval or sleep timer running in the main loop thread-safely.
   - The adjustment must take effect immediately (e.g., if the interval is adjusted downwards, or on the next tick).
4. **State Reporting:**
   - Publish the confirmed polling interval value back to the state topic: `<prefix>/sensor/sysmqttd_<hostname>/interval/state`.
   - Publish this state on startup as well as on every subsequent successful dynamic change.

## Acceptance Criteria
- Sending a valid numeric string payload (e.g., `"30"`) to `<prefix>/sensor/sysmqttd_<hostname>/interval/set` updates the daemon polling frequency.
- Invalid inputs (e.g., `"foo"`, `"-5"`, `"0"`) do not change the interval.
- Changes in polling intervals take effect without requiring a process restart or disconnecting from the broker.
- Confirmed state updates are verified via publication to `<prefix>/sensor/sysmqttd_<hostname>/interval/state`.
