# Implementation Plan: Safe Remote Commands for `sysmqttd`

This plan guides the implementation of the safe remote commands feature.

## Phase 1: Command Parsing Core & Tests [checkpoint: d6a7fbc]
Implement command string parsing and whitelisting, and verify it with unit tests.

- [x] Task: Implement command string parsing and whitelisting logic
- [x] Task: Write unit tests for command parser
- [x] Task: Conductor - User Manual Verification 'Phase 1: Command Parser' (Protocol in workflow.md)

## Phase 2: Event Loop Subscription Integration [checkpoint: 8087c8e]
Integrate subscriptions and whitelisted execution in the daemon main async loop.

- [x] Task: Add subscription handling in `run_with_shutdown`
- [x] Task: Integrate safe system command execution loops
- [x] Task: Run format, lint, and coverage gates
- [x] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
