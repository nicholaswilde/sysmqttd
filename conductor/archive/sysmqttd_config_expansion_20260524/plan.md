# Implementation Plan: Multi-Format Configuration File Manager for `sysmqttd`

This plan guides the implementation of the expanded layered configuration manager.

## Phase 1: Layered Config Loader & Unit Tests [checkpoint: a7544e5]
Implement the multi-format file loader, prefixed environment variable parser, and unit tests.

- [x] Task: Expand configuration data structures and serialization dependencies
    - [x] Add `serde_yaml` and `toml` (or standard `config` crate) to `Cargo.toml`
    - [x] Refactor `src/config.rs` to parse JSON, TOML, and YAML formats
- [x] Task: Parse environment variables with the `SYSMQTTD_` prefix
- [x] Task: Write comprehensive unit tests for layered configuration loading
    - [x] Test JSON parsing, TOML parsing, and YAML parsing
    - [x] Test `SYSMQTTD_` environment variable overrides
- [x] Task: Conductor - User Manual Verification 'Phase 1: Config Loading Core' (Protocol in workflow.md)

## Phase 2: CLI Integration and Audits [checkpoint: a7544e5]
Integrate the custom config path CLI flag and verify formatting, linting, and coverage.

- [x] Task: Add `-c` / `--config` flag parsing in `main.rs`
- [x] Task: Integrate config loading sequence with CLI arguments
- [x] Task: Audit formatting, linting, and 90%+ coverage gate (`task coverage`)
- [x] Task: Conductor - User Manual Verification 'Phase 2: Integration' (Protocol in workflow.md)
