# Specification: Unified Multi-Architecture DEB and RPM Packaging

This specification defines the requirements for adding unified multi-architecture Debian (.deb) and RedHat (.rpm) packaging to the `sysmqttd` release workflow.

## Overview
Automate packaging `sysmqttd` into production-ready `.deb` and `.rpm` files for all supported host architectures (`x86_64`, `aarch64`, `armv7`, `armv6`) in the GitHub Action release pipeline. The packages should run a common, robust installer script to set up system user accounts, directories with secure permissions, systemd service units, and initial configurations cleanly on target servers.

## Functional Requirements
1. **Multi-Architecture Matrix Compilation**:
   - Translate targets `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`, `armv7-unknown-linux-musleabihf`, and `arm-unknown-linux-gnueabihf` to their respective deb/rpm target architectures (`amd64`/`x86_64`, `arm64`/`aarch64`, `armhf`/`armhfp`, `armel`/`armv6hl`).
2. **Unified Staging Layout**:
   - Layout the binary under `/usr/bin/sysmqttd`.
   - Layout configuration templates under `/etc/sysmqttd/sysmqttd.toml.example`.
   - Layout service templates under `/usr/share/sysmqttd/`.
3. **OS-Agnostic Post-Install Script**:
   - Detect host capabilities (`groupadd`/`addgroup`, `useradd`/`adduser`).
   - Provision a secure `sysmqttd` system user with `nologin` shell.
   - Establish `/var/lib/sysmqttd` (state) and `/etc/sysmqttd` (config) folders with user permissions `chown sysmqttd:sysmqttd` and mode `750`.
   - Copy `sysmqttd.toml.example` to `sysmqttd.toml` only if no existing configuration is found.
   - Substitute placeholders (`{{SYSMQTTD_USER}}`, etc.) inside the systemd service template and write it to the appropriate systemd system path (e.g. `/lib/systemd/system` on Debian, `/usr/lib/systemd/system` on RHEL).
   - Proactively run `systemctl daemon-reload` if running on systemd.
4. **Pre-Uninstall/Post-Uninstall Cleanup**:
   - Stop and disable the service during removal.
   - Reload systemd daemon to clean up unit paths.

## Acceptance Criteria
- Valid `.deb` packages generated for all 4 targets.
- Valid `.rpm` packages generated for all 4 targets.
- Automated packaging running seamlessly in GitHub Action `release.yml`.
- Standard installation installs binary, configs, templates, and correctly configures system user/systemd units.
