# Implementation Plan: Command-Line Argument Equivalents for All Environment Variables

This plan guides the implementation of command-line argument equivalents for all environment variables.

## Phase 1: Core CLI Argument Parsing & Unit Tests [checkpoint: 88e6fec]
Implement expanded CLI parser parameters and verify via unit tests.

- [x] Task: Expand CLI parser parameters in `src/cli.rs`
    - [x] Support `-H` / `--host`
    - [x] Support `-P` / `--port`
    - [x] Support `-u` / `--user`
    - [x] Support `-w` / `--password`
    - [x] Support `-p` / `--prefix`
    - [x] Support `-i` / `--interface`
    - [x] Support `-s` / `--services`
- [x] Task: Write comprehensive unit tests for the CLI parser covering all new parameter flags
- [x] Task: Conductor - User Manual Verification 'Phase 1: CLI Parsing Core' (Protocol in workflow.md)

## Phase 2: Configuration Loader Integration & Audits [checkpoint: 88e6fec]
Integrate parameter arguments into the configuration loading sequence and verify code coverage.

- [x] Task: Integrate parameter flags in configuration loader sequence inside `src/config.rs` and `src/main.rs`
- [x] Task: Document all new options in the CLI usage output screen (`src/cli.rs`)
- [x] Task: Verify formatting, linting, and 90%+ code coverage gate (`task coverage`)
- [x] Task: Conductor - User Manual Verification 'Phase 2: Configuration Integration' (Protocol in workflow.md)
