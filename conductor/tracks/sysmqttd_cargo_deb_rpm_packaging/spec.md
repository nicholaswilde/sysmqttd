# Specification: Native Cargo Packaging (cargo-deb and cargo-generate-rpm)

This specification defines the migration of the Debian and RPM packaging process in the CI/CD pipeline to native Rust tools.

## Overview
Currently, the `release.yml` workflow installs Ruby, `rpm` tools, and `fpm` to generate `.deb` and `.rpm` release packages. This track replaces `fpm` with `cargo-deb` and `cargo-generate-rpm` for a more integrated, faster, and reliable Rust-native packaging process.

## Functional Requirements
1. **Debian Package Generation**:
   - Utilize `cargo-deb` configured inside `Cargo.toml` metadata.
   - Package systemd services, sudoers configurations, and user script hooks correctly.
2. **RPM Package Generation**:
   - Utilize `cargo-generate-rpm` configured inside `Cargo.toml` or via command parameters.
   - Support installation paths, systemd integrations, and scripts.
3. **CI/CD Integration**:
   - Clean up FPM installation actions.
   - Install `cargo-deb` and `cargo-generate-rpm` inside the build matrix.
   - Multi-arch support matching the target platforms (`x86_64`, `aarch64`, `armv7`, `armv6`).

## Acceptance Criteria
- Releases successfully build correct `.deb` and `.rpm` files for all architectures.
- The packaging scripts (`post_install.sh` and `pre_uninstall.sh`) are executed properly upon package install/uninstall.
- Eliminate dependency on Ruby and FPM from the release workflow.
