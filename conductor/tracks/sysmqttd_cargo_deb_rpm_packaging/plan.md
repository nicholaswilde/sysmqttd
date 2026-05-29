# Implementation Plan: Native Cargo Packaging with cargo-deb and cargo-generate-rpm

This plan guides the conversion of the package release workflow from ruby-fpm to cargo-deb and cargo-generate-rpm.

## Phase 1: Packaging Configurations & Local Validation [checkpoint: 7a8136d]

Configure package metadata and test local package builds.

- [x] Task: Add deb packaging metadata `[package.metadata.deb]` to `Cargo.toml`
- [x] Task: Add rpm packaging metadata/attributes in `Cargo.toml` or configure arguments for `cargo-generate-rpm`
- [x] Task: Test generation of deb packages locally using `cargo deb`
- [x] Task: Test generation of rpm packages locally using `cargo generate-rpm`
- [x] Task: Configure packaging scripts to handle service stop, update, and restart during upgrades without post-upgrade messages
- [x] Task: Conductor - User Manual Verification 'Phase 1: Local Packaging Configs'


## Phase 2: Workflow Refactoring & Cross Compilation

Refactor the CI/CD release workflow to use the native Rust tools.

- [ ] Task: Update `.github/workflows/release.yml` to replace FPM installation and packaging steps
- [ ] Task: Ensure multi-architecture support (x86_64, aarch64, armv7, armv6) for both tools
- [ ] Task: Validate the refactored release workflow on push/tag events
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Workflow Refinement'
