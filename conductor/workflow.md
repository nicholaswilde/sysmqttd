# Workflow: `sysmqttd` Development Lifecycle

This document defines the strict development workflow, verification gates, and checkpoint procedures for the `sysmqttd` daemon project.

## 1. Development Principles
*   **Test-Driven Development (TDD):** For core logical components (configuration parser, discovery payload serialization, hostname helper functions), write unit tests before implementing the code.
*   **Target Simulation / Local Verification:** Test logical execution locally (on the development workstation) against a local MQTT broker (e.g., Mosquitto).
*   **Cross-Compilation First:** Every phase's implementation must be checked using `cross` to ensure no architecture-specific compilation bugs are introduced.
*   **Aggressive Size/Memory Optimization Gates:** Periodically check binary size and target memory metrics.

## 2. Universal File Resolution Protocol
When searching for files within the repository:
1.  Check the workspace root first.
2.  Follow the pathing specified in `conductor/tracks.md`.

## 3. Phase Completion Verification and Checkpointing Protocol
At the end of each Phase in the implementation plan, the agent must perform the following:
1.  **Verify All Automated Tests Pass:** Run local tests using `cargo test`.
2.  **Execute Cross-Compilation Check:** Verify target compiles via `cross build --target arm-unknown-linux-gnueabihf`.
3.  **Perform Manual Verification Steps:**
    *   Start local `mosquitto` container or service.
    *   Run compiled binary locally (connecting to local broker).
    *   Use `mosquitto_sub` to verify correct payloads are published to topics.
4.  **User Manual Verification:** Present the results clearly to the user, list the manual verification steps followed, and ask:
    `Does this meet your expectations? Please confirm with yes or provide feedback on what needs to be changed.`
5.  **Create Checkpoint Commit:** Stage and commit all changes with message `conductor(checkpoint): Checkpoint end of Phase X`.
6.  **Attach Auditable Verification Report:** Write a brief validation summary into git notes.
7.  **Record Phase Checkpoint SHA:** Update `plan.md` by appending `[checkpoint: <7-char-sha>]` to the phase header.

## 4. Coding Styleguides
*   Refer to `conductor/code_styleguides/rust.md` for Rust coding standards.

## 5. Development Commands

### Setup
```bash
# Verify docker is running (for cross)
docker --version

# Install cross-compilation target toolchain and cross utility
rustup target add arm-unknown-linux-gnueabihf
cargo install cross --git https://github.com/cross-rs/cross
```

### Daily Development
```bash
# Run unit tests locally
cargo test

# Fast local build checks
cargo check

# Run the project locally (using environment variables)
MQTT_HOST=localhost MQTT_PORT=1883 cargo run
```

### Before Committing
```bash
# Run rustfmt
cargo fmt --all -- --check

# Run clippy for strict linting
cargo clippy --all-targets --all-features -- -D warnings

# Build the final release cross-compiled binary
cross build --target arm-unknown-linux-gnueabihf --release
```

## 6. Definition of Done
A task is complete when:
1. Code compiles without warnings on both host and `arm-unknown-linux-gnueabihf` target.
2. Unit tests are written and passing.
3. Code is formatted (`cargo fmt`) and clippy is clean.
4. Binary size and optimization criteria are verified.
5. Code changes are committed with conventional commit messages.
6. The `README.md` and `sysmqttd.toml.example` at the project root are updated to accurately document any new or changed features, options, configurations, and parameters.

## 7. General Project Specifications

*   **Release Compilation Check:** In addition to unit tests, whenever a feature is implemented or modified, the project should be verified in release mode locally to guarantee clean compilation:
    ```bash
    cargo build --release
    ```
*   **Issue Resolution Protocol:**
    *   All open issues affected by a change must be reviewed.
    *   Any regressions introduced by the change must be resolved.
    *   Changes can be committed and merged only after verifying all tests pass and builds succeed.
*   **Continuous Integration (CI):** The CI pipeline runs `cargo test` and `cargo build --release` on every pull request. Pull requests failing these checks will be blocked from merging.
*   **Documentation Standards:**
    *   Update relevant documentation (e.g., README, module docs) to reflect new functionality.
    *   Ensure the documentation builds cleanly without errors.

