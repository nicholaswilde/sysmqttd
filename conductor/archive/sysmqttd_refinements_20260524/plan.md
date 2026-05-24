# Implementation Plan: `sysmqttd` Refinements & Testing Suite

This plan guides the implementation of the new testing, deployment, and taskrunner requirements.

## Phase 1: Taskfile Scaffolding & README Warning [checkpoint: ca3845f]
Draft basic configurations and warnings.

- [x] Task: Update `README.md` at project root with development warning at the top.
- [x] Task: Create initial `Taskfile.yml` with formatting, linting, and default recipes.
- [x] Task: Conductor - User Manual Verification 'Phase 1 Scaffolding' (Protocol in workflow.md)

## Phase 2: Daemon Refactoring & Unit Tests [checkpoint: ae99c42]
Extract the async telemetry poller out of `main.rs` to allow robust testing.

- [x] Task: Design and create `src/daemon.rs` housing telemetry collection and connection event loops.
- [x] Task: Simplify `src/main.rs` to boot config and instantiate the new `Daemon` structure.
- [x] Task: Write unit tests verifying core `Daemon` operations.
- [x] Task: Conductor - User Manual Verification 'Phase 2 Daemon Refactor' (Protocol in workflow.md)

## Phase 3: Integration Tests & 90% Coverage Gate [checkpoint: f2cca22]
Enhance the testing infrastructure to enforce quality gates.

- [x] Task: Create `tests/integration_test.rs` to verify daemon behavior against mock brokers.
- [x] Task: Configure the coverage task inside `Taskfile.yml` to enforce a 90% line coverage threshold.
- [x] Task: Verify that `task coverage` runs successfully and passes the 90%+ limit check.
- [x] Task: Conductor - User Manual Verification 'Phase 3 Coverage Gate' (Protocol in workflow.md)

## Phase 4: Docker & Compose Configurations [checkpoint: df46171]
Implement multi-architecture docker containers and sandbox compose configs.

- [x] Task: Create multi-stage `Dockerfile`.
- [x] Task: Create local orchestration sandbox in `compose.yaml`.
- [x] Task: Spin up compose stack and verify logging output streams cleanly.
- [x] Task: Conductor - User Manual Verification 'Phase 4 Dockerization' (Protocol in workflow.md)

## Phase 5: Multi-Architecture Cross-Compilation [checkpoint: 7144c3c]
Integrate multiple compilation recipes into the task runner.

- [x] Task: Configure `build-all` task in `Taskfile.yml` compiling for ARMv6, ARMv7, ARM64, and AMD64.
- [x] Task: Execute the build task and verify each stripped target binary using `file`.
- [x] Task: Conductor - User Manual Verification 'Phase 5 Cross-Compilation' (Protocol in workflow.md)

## Phase 6: Final Verification & Conductor Updates [checkpoint: e708d41]
Audit the completed refinements track.

- [x] Task: Run all `Taskfile.yml` recipes to verify workflow compliance.
- [x] Task: Generate track documentation updates and walkthroughs.
- [x] Task: Conductor - User Manual Verification 'Phase 6 Integration Audit' (Protocol in workflow.md)
