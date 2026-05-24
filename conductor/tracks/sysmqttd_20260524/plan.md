# Implementation Plan: `sysmqttd` System Monitor

This plan breaks down the development of the `sysmqttd` system monitoring daemon into clear, incremental phases. Each phase concludes with automated and manual verification checkpoints.

## Phase 1: Environment Scaffolding & Cargo Setup [checkpoint: db5ab4a]
Set up the workspace, dependency configurations, and cross-compilation target suite.

- [x] Task: Create new Cargo binary project `sysmqttd`.
- [x] Task: Configure `Cargo.toml` with `sysinfo`, `rumqttc`, `serde`, and `serde_json`.
- [x] Task: Set up `Cargo.toml` profiles for Link-Time Optimization (`lto = true`), single codegen unit (`codegen-units = 1`), panic abort (`panic = "abort"`), and size optimizations (`opt-level = "z"`, `strip = true`).
- [x] Task: Configure `cross` (Cross-compilation) for target `arm-unknown-linux-gnueabihf` by creating a `Cross.toml` if needed.
- [x] Task: Build a minimal "Hello World" binary via `cross` and verify the output binary architecture using `file`.
- [x] Task: Conductor - User Manual Verification 'Phase 1 Setup' (Protocol in workflow.md)

## Phase 2: Configuration & MQTT Client Loop [checkpoint: 8f483a7]
Implement configuration loading and the asynchronous MQTT client connection loop.

- [x] Task: Implement configuration module supporting environment variables and a fallback TOML file.
- [x] Task: Write unit tests verifying configuration parser, fallback defaults, and environment overrides.
- [x] Task: Implement async connection logic using `rumqttc::AsyncClient` and loop handler.
- [x] Task: Incorporate connection backoff and auto-reconnect retry strategies.
- [x] Task: Test async client loop connection locally against a Mock MQTT broker.
- [x] Task: Conductor - User Manual Verification 'Phase 2 MQTT Loop' (Protocol in workflow.md)

## Phase 3: Home Assistant MQTT Discovery
Implement startup retained discovery publishing registering all metrics under a unified parent device.

- [ ] Task: Model HA Discovery JSON schema structs for CPU Temp, RAM usage, and Disk usage sensors.
- [ ] Task: Write unit tests validating Discovery struct serialization formats.
- [ ] Task: Implement hostname retrieval helper and discovery payload publishing logic.
- [ ] Task: Verify that discovery payloads are published as `retained` messages on startup.
- [ ] Task: Conductor - User Manual Verification 'Phase 3 Discovery' (Protocol in workflow.md)

## Phase 4: Telemetry Metrics Collection
Implement non-blocking telemetry acquisition for CPU Temperature, RAM, and Disk metrics.

- [ ] Task: Configure minimized `sysinfo` features (`default-features = false`, only enabling necessary platform systems) and implement RAM & Disk percent utilization functions.
- [ ] Task: Implement CPU Temperature query via direct thermal sensor `/sys/class/thermal/thermal_zone0/temp` (default on Raspberry Pi / DietPi) with secondary fallback.
- [ ] Task: Write unit tests for memory, disk, and temp reader functions with mock values.
- [ ] Task: Implement the main 60-second telemetry polling and async publish loop.
- [ ] Task: Conductor - User Manual Verification 'Phase 4 Telemetry' (Protocol in workflow.md)

## Phase 5: Size & Resource Optimization
Optimize binary sizes and runtime memory footprint.

- [ ] Task: Compile cross-compiled release build using `cross` and verify binary is stripped.
- [ ] Task: Measure final compiled binary size (target < 2MB).
- [ ] Task: Run performance profiling or runtime memory monitoring (Valgrind or RSS tracking) if possible to verify RAM consumption target (< 8MB RAM).
- [ ] Task: Conductor - User Manual Verification 'Phase 5 Optimization' (Protocol in workflow.md)

## Phase 6: Systemd Service Installation & Deployment
Draft systemd unit configuration, service deployment, and installation scripts.

- [ ] Task: Create `sysmqttd.service` systemd service unit template running as a non-root standard user.
- [ ] Task: Document configuration, target installation directories (e.g. `/usr/bin/sysmqttd` and `/etc/sysmqttd/`), and systemd setup commands.
- [ ] Task: Run complete automated test suite locally to verify code correctness.
- [ ] Task: Conductor - User Manual Verification 'Phase 6 Deployment' (Protocol in workflow.md)
