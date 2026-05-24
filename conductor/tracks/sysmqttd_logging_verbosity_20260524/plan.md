# Implementation Plan: Logging Verbosity Control for `sysmqttd`

This plan guides the implementation of logging verbosity control.

## Phase 1: CLI and Configuration Verbosity Flag [checkpoint: 066bcfb]
Add configuration parsing for verbose logging.

- [x] Task: Update `Config` and `CliOverrides` to support verbosity
    - [x] Add `verbose` field to `Config`, `FileConfig`, and `CliOverrides`
    - [x] Support `--verbose` CLI parameter and `SYSMQTTD_VERBOSE` environment variable
- [x] Task: Write unit tests for verbosity config overrides
- [x] Task: Conductor - User Manual Verification 'Phase 1: Configuration' (Protocol in workflow.md)

## Phase 2: Logging Toggle Integration & Verification [checkpoint: fcf1ec1]
Integrate verbosity checks in daemon loops.

- [x] Task: Conditionally print telemetry publication payloads in daemon state loops
- [x] Task: Conditionally print MQTT event loop packets (`Event::Incoming` / `Event::Outgoing`)
- [x] Task: Confirm formatting, clippy lints, and 90%+ code coverage
- [x] Task: Conductor - User Manual Verification 'Phase 2: Logging Verification' (Protocol in workflow.md)
