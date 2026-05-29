# Tech Stack: `sysmqttd`

This document specifies the technologies, crates, and aggressive optimization profiles selected for building the `sysmqttd` daemon on ARMv6 hardware.

## 1. Programming Language
*   **Language:** Rust (latest stable).
*   **Target Triple:** `arm-unknown-linux-gnueabihf` (ARMv6 hard float, supporting Raspberry Pi Zero W / ARM1176JZF-S).

## 2. Core Dependencies (Crates)
*   **`sysinfo`:**
    *   **Feature Minimized:** Configured with `default-features = false`. Only the necessary features (`system`, `disk`) are enabled to reduce compiled size and memory allocation.
*   **`rumqttc`:**
    *   **Async Client:** Uses `rumqttc::AsyncClient` and `rumqttc::EventLoop` for a fully non-blocking asynchronous event loop.
*   **`serde` and `serde_json`:**
    *   Used for generating clean, flat payload structures for Home Assistant Discovery and Telemetry states.
*   **`toml` and `serde_yaml`:**
    *   Used for parsing and deserializing TOML, YAML, and JSON configuration files.
*   **`tokio`:**
    *   Used as the async runtime. Configured with the single-threaded scheduler (`rt` feature only) to keep runtime overhead to an absolute minimum.


## 3. Cargo.toml Optimization Profile
To achieve our target resource limits (RAM < 8MB RSS, Binary Size < 2MB), the following configuration is defined in `Cargo.toml`:

```toml
[profile.release]
# Link-Time Optimization (LTO)
lto = true

# Compile all crates in a single codegen unit to maximize LTO optimization opportunities
codegen-units = 1

# Abort on panic to eliminate stack unwinding tables and landing pads
panic = "abort"

# Optimize aggressively for binary size
opt-level = "z"

# Strip all symbols and debug info from the final binary
strip = true
```

## 4. Cross-Compilation Setup
To build the ARMv6 binary on x86_64 development machines:
*   Use `cross` (containerized cross-compilation tool).
*   Require a running `docker` or `podman` engine.
*   Compilation command:
    ```bash
    cross build --target arm-unknown-linux-gnueabihf --release
    ```

## 5. Deployment & Packaging
*   **Native Cargo Packaging (`cargo-deb` & `cargo-generate-rpm`):** Integrated directly into `Cargo.toml` metadata and the GitHub release workflow. They generate highly optimized `.deb` and `.rpm` packages natively for all target architectures without external Ruby or FPM dependencies.
*   **Distro-Agnostic Installer Scripts:** Custom scripts (`post_install.sh` and `pre_uninstall.sh`) and maintainer scripts (`postinst` and `prerm` for DEB) are run by host package managers to automate system user/group creation (`sysmqttd`), secure file storage, copy configuration templates, and cleanly stop, update, and restart the active systemd service units. Upgrades are processed silently without success messages.

## 6. Testing & CI Stack
*   **`testcontainers`:** Async Rust container orchestration, dynamically spinning up transient, authentication-free `eclipse-mosquitto` container instances inside integration tests to avoid port binding conflicts.
*   **`cargo-llvm-cov`:** Source-based code coverage tool for Rust, enforcing a strict 90% line gate in the CI/CD pipeline.

