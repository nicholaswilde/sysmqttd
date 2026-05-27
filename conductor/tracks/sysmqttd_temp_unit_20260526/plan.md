# Implementation Plan: Temperature Unit Selection (Celsius or Fahrenheit)

This plan guides the implementation of the temperature unit selection configuration and conversion logic.

## Phase 1: Configuration and CLI Parsing
Implement the config file and command-line option parsing with unit tests.

- [ ] Task: Add `temperature_unit` to configuration structures in `src/config.rs` and `src/cli.rs`
    - [ ] Update `Config` struct to hold unit value
    - [ ] Add `-u` / `--unit` / `--temperature-unit` CLI parser flags
    - [ ] Set unit default to Fahrenheit (`"F"`) when not configured
- [ ] Task: Write TDD unit tests in `src/config.rs` and `src/cli.rs`
    - [ ] Test config parsing from TOML
    - [ ] Test CLI flag overrides
    - [ ] Test environment variable `SYSMQTTD_TEMPERATURE_UNIT` overrides
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Configuration & CLI' (Protocol in workflow.md)

## Phase 2: Conversion and Discovery Integration
Implement C-to-F conversion, adjust discovery registration, and run automated verification checks.

- [ ] Task: Implement temperature conversion in `src/telemetry.rs`
    - [ ] Convert temperature from Celsius to Fahrenheit if `"F"` is active
- [ ] Task: Adjust Discovery payload unit in `src/discovery.rs` and `src/daemon.rs`
    - [ ] Pass the active unit to `new_cpu_temp` and update the `unit_of_measurement`
- [ ] Task: Add and run automated tests
    - [ ] Add unit tests in `telemetry.rs` for C-to-F conversion logic
    - [ ] Add unit tests in `discovery.rs` for dynamic unit auto-discovery payload
    - [ ] Run format, clippy, and cross-compilation checks
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Conversion & Discovery' (Protocol in workflow.md)
