# Implementation Plan: Container-Driven Integration Testing

This plan outlines the phases to implement fully isolated, container-driven integration tests using `testcontainers`.

## Phase 1: Dependency Injection & Boilerplate Setup [checkpoint: 663266b]

Introduce `testcontainers` dev-dependencies and establish container startup boilerplate.

- [x] Task: Add `testcontainers` with `tokio` feature as a dev-dependency in `Cargo.toml`
- [x] Task: Implement container orchestration helper in a new integration test module or in `tests/integration_test.rs`
- [x] Task: Dynamically resolve port mappings to prevent port binding conflicts
- [x] Task: Conductor - User Manual Verification 'Phase 1: Container Integration Boilerplate'

## Phase 2: Telemetry & Discovery Validation Assertions

Migrate existing test cases to the dynamic broker and add strict assertions verifying product specifications.

- [x] Task: Refactor `tests/integration_test.rs` to run against the dynamic Mosquitto container
- [x] Task: Implement assertions verifying MQTT Client ID conforms to `sysmqttd_<hostname>`
- [x] Task: Implement assertions verifying state payload formats (flat JSON, numeric, and boolean flags)
- [x] Task: Implement assertions verifying Home Assistant Discovery payloads (device class, templates, components)
- [x] Task: Conductor - User Manual Verification 'Phase 2: Assertions and Core Validation'

## Phase 3: CI/CD Pipeline & Taskfile Streamlining

Integrate and validate the new test suite across local environments and remote workflows.

- [ ] Task: Clean up external docker commands from `Taskfile.yml` `test` tasks
- [ ] Task: Verify that `cargo test` runs successfully without manual broker orchestration
- [ ] Task: Run clippy and cross-compilation checks to ensure build hygiene
- [ ] Task: Conductor - User Manual Verification 'Phase 3: CI/CD & Pipeline Streamlining'
