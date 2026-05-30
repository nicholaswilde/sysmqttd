# Implementation Plan: Dynamic Polling Interval Adjustment via MQTT Command

This plan outlines the design and implementation steps for dynamically modifying the telemetry polling interval during execution via incoming MQTT messages.

## Phase 1: Subscription & Validation [checkpoint: ed3ebe9]

Implement the subscription to the command topic and parsing/validation logic.

- [x] Task: Set up MQTT subscription to `<prefix>/sensor/sysmqttd_<hostname>/interval/set`
- [x] Task: Parse and validate payloads as valid bounds-checked integers (1s to 86400s)
- [x] Task: Integrate log reporting for parsed commands and rejected payloads
- [x] Task: Conductor - User Manual Verification 'Phase 1: Dynamic Interval Subscriptions'

## Phase 2: Dynamic Loop Integration & Verification [checkpoint: ed3ebe9]

Integrate dynamic interval updates in the main loop and publish state feedback.

- [x] Task: Implement a thread-safe message/signal channel or shared state (e.g., `Arc<RwLock<Duration>>`) for the main timer loop
- [x] Task: Adapt the tokio main interval loop to check and update its interval dynamic duration
- [x] Task: Publish state confirmations to `<prefix>/sensor/sysmqttd_<hostname>/interval/state`
- [x] Task: Write integration/unit tests validating in-flight loop interval shifts
- [x] Task: Conductor - User Manual Verification 'Phase 2: Live Loop Update & Tests'
