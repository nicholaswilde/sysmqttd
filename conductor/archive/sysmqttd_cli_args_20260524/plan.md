# Implementation Plan: Version and Help CLI Arguments for `sysmqttd`

This plan guides the implementation of the version and help command line arguments feature for the `sysmqttd` daemon.

## Phase 1: Argument Parsing Core and CLI Unit Tests
Implement the logic to scan and parse CLI arguments and verify it via unit tests.

- [ ] Task: Implement argument scanning logic in a new function or module
    - [ ] Add `parse_arguments(args: Vec<String>) -> Result<CliAction, String>` helper
    - [ ] Define the `CliAction` enum representing Boot, PrintHelp, PrintVersion, or error
- [ ] Task: Write unit tests for CLI argument parsing
    - [ ] Test that passing `-h` or `--help` returns `CliAction::PrintHelp`
    - [ ] Test that passing `-v` or `--version` returns `CliAction::PrintVersion`
    - [ ] Test that passing invalid arguments returns an error message
    - [ ] Test that empty/no arguments default to `CliAction::Boot`
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Argument Parsing Core' (Protocol in workflow.md)

## Phase 2: CLI Integration in Daemon Driver
Integrate CLI argument scanning into `main.rs` to control the boot sequence.

- [x] Task: Modify `src/main.rs` to process arguments before configuration boot
    - [x] Extract `std::env::args()` and call `parse_arguments`
    - [x] If `CliAction::PrintHelp`, print usage info to stdout and exit(0)
    - [x] If `CliAction::PrintVersion`, print dynamic Cargo PKG version to stdout and exit(0)
    - [x] If `CliAction::Boot`, proceed with the standard config loading and daemon boot sequence
    - [x] If error, print the error to stderr and exit(1)
- [x] Task: Verify overall command line options and unit test coverage
    - [x] Run `task fmt` and `task lint` to verify code style and lints
    - [x] Run `task coverage` to ensure we maintain at least 90% total line coverage
- [x] Task: Conductor - User Manual Verification 'Phase 2: CLI Integration in Daemon Driver' (Protocol in workflow.md)

## Phase 3: Final Verification & Cross-Compilations
Verify the implementation under the Taskfile framework and perform multi-architecture builds.

- [ ] Task: Run integration tests and execute final cross-compilations
    - [ ] Verify that running `./sysmqttd -h` prints usage without needing env variables
    - [ ] Run `task build-all` to ensure all targets (ARMv6, ARMv7, ARM64, AMD64) compile cleanly
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Final Verification' (Protocol in workflow.md)
