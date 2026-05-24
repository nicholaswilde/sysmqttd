# Specification - `sysmqttd` Refinements & Testing Suite

## 1. Overview
This specification refines `sysmqttd` by introducing a Task runner (`go-task`), expanding cross-compilation targets, containerization (`Dockerfile` and `compose.yaml`), establishing an integration testing harness, and enforcing a strict 90%+ code coverage gate.

## 2. Refined Requirements & Scope

### 2.1 Task Runner (`Taskfile.yml`)
Introduce `Taskfile.yml` configured with the following tasks:
*   `format`: Format Rust files using `cargo fmt`.
*   `lint`: Perform strict lint audits using `cargo clippy`.
*   `test`: Execute local unit tests.
*   `test-integration`: Run integration tests.
*   `coverage`: Execute `cargo llvm-cov` and enforce `--fail-under-lines 90`.
*   `build-all`: Cross-compile stripping release binaries for all four architectures.

### 2.2 Target Architectures
Cross-compile optimized release binaries for the following targets:
1.  `arm-unknown-linux-gnueabihf` (ARMv6 hard float, Raspberry Pi Zero W / ARM1176JZF-S)
2.  `armv7-unknown-linux-gnueabihf` (ARMv7 hard float, Raspberry Pi 2/3 / Cortex-A7)
3.  `aarch64-unknown-linux-gnu` (ARM64, Raspberry Pi 3/4/5 / Cortex-A53/72/76)
4.  `x86_64-unknown-linux-gnu` (AMD64 / standard PC/server architecture)

### 2.3 Unit and Integration Testing
*   **Daemon Refactoring:** Move the main loop from `src/main.rs` to `src/daemon.rs` to allow testing connection timeouts, autodiscovery setups, and message publications without running the main binary.
*   **Integration Tests:** Establish a `tests/` directory with `integration_test.rs` which verifies telemetry loops, payload structures, and message flows against mock brokers.

### 2.4 Code Coverage Gate
*   Maintain a strict **90%+ line coverage** threshold across the entire project.
*   Enforce this using `cargo-llvm-cov` with `--fail-under-lines 90` in the `Taskfile.yml` coverage recipe.

### 2.5 Containerization (Dockerfile & compose.yaml)
*   **Dockerfile:** Create a multi-stage Dockerfile that compiles the Rust code cleanly or packages the compiled target binary in a minimal secure base image.
*   **compose.yaml:** Include a compose setup spinning up both the `sysmqttd` container and a local `eclipse-mosquitto` broker in an isolated bridge network to enable immediate local debugging.

### 2.6 README Warning
Insert a warning block at the top of `README.md` stating:
> [!WARNING]
> **This is a development version at version v0.1.\* and things may change at any time.**
