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
*   **FPM (Effing Package Management):** Used inside the GitHub release workflow to package the precompiled and stripped binaries into `.deb` and `.rpm` files for all target architectures.
*   **Distro-Agnostic Installer Scripts:** Custom scripts (`post_install.sh` and `pre_uninstall.sh`) are run by the host package managers to automate the creation of the system user (`sysmqttd`), secure file storage (`/var/lib/sysmqttd` and `/etc/sysmqttd`), copy default configuration templates, and cleanly register and enable the active systemd service units.

