# Implementation Plan: Command-Line Argument Equivalents for All Environment Variables

This plan guides the implementation of command-line argument equivalents for all environment variables.

## Phase 1: Core CLI Argument Parsing & Unit Tests
Implement expanded CLI parser parameters and verify via unit tests.

- [ ] Task: Expand CLI parser parameters in `src/cli.rs`
    - [ ] Support `-H` / `--host`
    - [ ] Support `-P` / `--port`
    - [ ] Support `-u` / `--user`
    - [ ] Support `-w` / `--password`
    - [ ] Support `-p` / `--prefix`
    - [ ] Support `-i` / `--interface`
    - [ ] Support `-s` / `--services`
- [ ] Task: Write comprehensive unit tests for the CLI parser covering all new parameter flags
- [ ] Task: Conductor - User Manual Verification 'Phase 1: CLI Parsing Core' (Protocol in workflow.md)

## Phase 2: Configuration Loader Integration & Audits
Integrate parameter arguments into the configuration loading sequence and verify code coverage.

- [ ] Task: Integrate parameter flags in configuration loader sequence inside `src/config.rs` and `src/main.rs`
- [ ] Task: Document all new options in the CLI usage output screen (`src/cli.rs`)
- [ ] Task: Verify formatting, linting, and 90%+ code coverage gate (`task coverage`)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Configuration Integration' (Protocol in workflow.md)
