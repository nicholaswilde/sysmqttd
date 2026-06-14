# Plan: Fan Speed Monitoring

## Phase 1: Configuration & CLI Flag Support [checkpoint: d6818a3]
- [x] Task: Write tests for CLI parsing, configuration parsing, and environment overrides for `no_fan`.
    - [x] Write unit tests verifying `--no-fan` sets `no_fan = true`.
    - [x] Write unit tests verifying `SYSMQTTD_NO_FAN=true` sets `no_fan = true`.
    - [x] Write unit tests verifying configuration file parsing sets `no_fan`.
- [x] Task: Implement `no_fan` configuration support.
    - [x] Update config structure in `src/config.rs` to include `no_fan` field.
    - [x] Update CLI arguments parsing in `src/cli.rs` to support `--no-fan`.
    - [x] Update environment variable reading in `src/config.rs`.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Configuration & CLI Flag Support' (Protocol in workflow.md)

## Phase 2: Fan Speed Telemetry Gathering [checkpoint: 81a130d]
- [x] Task: Write tests for discovering and reading fan speeds.
    - [x] Write unit test verifying discovery of multiple mock fans in a temp directory.
    - [x] Write unit test verifying mock fallback to `fan_1` at `1200` RPM when no fans are discovered in non-root `sysfs_root`.
    - [x] Write unit test verifying no fans are collected when `no_fan` is set to `true`.
- [x] Task: Implement fan speed telemetry collection.
    - [x] Update `TelemetryCollector` in `src/telemetry.rs` to store `no_fan` flag.
    - [x] Implement `read_fan_speeds` logic in `src/telemetry.rs` to search for `/sys/class/hwmon/hwmon*/fan*_input`.
    - [x] Update `TelemetryState` in `src/telemetry.rs` to include flattened extra metrics map and populate it with fan speeds in `collect`.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Fan Speed Telemetry Gathering' (Protocol in workflow.md)

## Phase 3: Home Assistant Discovery & Payload Integration
- [ ] Task: Write tests for MQTT discovery payloads and serialization.
    - [ ] Write unit test verifying discovery configuration payload generated for a fan speed sensor.
    - [ ] Write unit test verifying serialization of `TelemetryState` includes flattened fan keys.
- [ ] Task: Implement discovery payload generation and publishing.
    - [ ] Update `src/discovery.rs` to add `new_fan_speed` constructor for `DiscoveryPayload`.
    - [ ] Update `src/daemon.rs` to dynamically discover fans and publish auto-discovery configuration messages for each discovered fan on startup.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Home Assistant Discovery & Payload Integration' (Protocol in workflow.md)

## Phase 4: Integration Verification, Formatting & Documentation
- [ ] Task: Integration Verification.
    - [ ] Run full test suite (`cargo test`) locally.
    - [ ] Check cross-compilation target (`cross build --target arm-unknown-linux-gnueabihf`).
    - [ ] Run clippy and formatting checks.
- [ ] Task: Update Documentation & Configuration Examples.
    - [ ] Update `README.md` to document the new fan speed monitoring feature and the `--no-fan` toggle options.
    - [ ] Update `sysmqttd.toml.example` to document the new `no_fan` configuration option.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Integration Verification, Formatting & Documentation' (Protocol in workflow.md)
