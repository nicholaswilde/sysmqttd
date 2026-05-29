# Specification: Jittered Exponential Reconnection Backoff

This specification defines the requirements for adding a jittered exponential backoff algorithm to `sysmqttd`'s MQTT reconnection loop.

## Overview
Prevents rapid reconnection hammering and log spamming when the network drops out or the Home Assistant MQTT broker is restarted, avoiding "thundering herd" conditions.

## Functional Requirements
1. **Configurable Reconnection Policy:**
   - Configure a dynamic backoff strategy for `rumqttc`'s connection client.
   - Initial retry delay must default to `2` seconds.
   - Delay must double on each consecutive failure up to a configurable maximum ceiling (default `300` seconds).
2. **Full Jitter Calculation:**
   - Inject randomized variance (full jitter) to prevent synchronous retry attempts across multiple daemon instances:
     `actual_delay = random(0, min(max_delay, initial_delay * 2^retries))`
3. **Log Quiet State Integration:**
   - Connection failure logging must be throttled to prevent journal-spamming during long-term broker downtime (e.g. only logging retry milestones like 1, 5, 10, etc., or once every 5 minutes).

## Acceptance Criteria
- Backoff delays accurately double and introduce randomized variance under simulated network timeouts.
- Zero rapid-retry infinite loop behavior on startup broker unreachable conditions.
- Test coverage validates correct backoff mathematical progression.
