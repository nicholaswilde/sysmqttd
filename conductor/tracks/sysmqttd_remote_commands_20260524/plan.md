# Implementation Plan: Safe Remote Commands for `sysmqttd`

This plan guides the implementation of the safe remote commands feature.

## Phase 1: Command Parsing Core & Tests
Implement command string parsing and whitelisting, and verify it with unit tests.

- [ ] Task: Implement command string parsing and whitelisting logic
- [ ] Task: Write unit tests for command parser
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Command Parser' (Protocol in workflow.md)

## Phase 2: Event Loop Subscription Integration
Integrate subscriptions and whitelisted execution in the daemon main async loop.

- [ ] Task: Add subscription handling in `run_with_shutdown`
- [ ] Task: Integrate safe system command execution loops
- [ ] Task: Run format, lint, and coverage gates
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
