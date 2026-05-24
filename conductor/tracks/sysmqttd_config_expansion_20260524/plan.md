# Implementation Plan: Multi-Format Configuration File Manager for `sysmqttd`

This plan guides the implementation of the expanded layered configuration manager.

## Phase 1: Layered Config Loader & Unit Tests
Implement the multi-format file loader, prefixed environment variable parser, and unit tests.

- [ ] Task: Expand configuration data structures and serialization dependencies
    - [ ] Add `serde_yaml` and `toml` (or standard `config` crate) to `Cargo.toml`
    - [ ] Refactor `src/config.rs` to parse JSON, TOML, and YAML formats
- [ ] Task: Parse environment variables with the `SYSMQTTD_` prefix
- [ ] Task: Write comprehensive unit tests for layered configuration loading
    - [ ] Test JSON parsing, TOML parsing, and YAML parsing
    - [ ] Test `SYSMQTTD_` environment variable overrides
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Config Loading Core' (Protocol in workflow.md)

## Phase 2: CLI Integration and Audits
Integrate the custom config path CLI flag and verify formatting, linting, and coverage.

- [ ] Task: Add `-c` / `--config` flag parsing in `main.rs`
- [ ] Task: Integrate config loading sequence with CLI arguments
- [ ] Task: Audit formatting, linting, and 90%+ coverage gate (`task coverage`)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
