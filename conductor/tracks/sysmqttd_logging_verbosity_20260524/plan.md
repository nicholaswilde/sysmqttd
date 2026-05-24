# Implementation Plan: Logging Verbosity Control for `sysmqttd`

This plan guides the implementation of logging verbosity control.

## Phase 1: CLI and Configuration Verbosity Flag
Add configuration parsing for verbose logging.

- [ ] Task: Update `Config` and `CliOverrides` to support verbosity
    - [ ] Add `verbose` field to `Config`, `FileConfig`, and `CliOverrides`
    - [ ] Support `--verbose` CLI parameter and `SYSMQTTD_VERBOSE` environment variable
- [ ] Task: Write unit tests for verbosity config overrides
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Configuration' (Protocol in workflow.md)

## Phase 2: Logging Toggle Integration & Verification
Integrate verbosity checks in daemon loops.

- [ ] Task: Conditionally print telemetry publication payloads in daemon state loops
- [ ] Task: Conditionally print MQTT event loop packets (`Event::Incoming` / `Event::Outgoing`)
- [ ] Task: Confirm formatting, clippy lints, and 90%+ code coverage
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Logging Verification' (Protocol in workflow.md)
