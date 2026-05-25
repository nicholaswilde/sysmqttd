# Implementation Plan: Unified Multi-Architecture DEB and RPM Packaging

This plan details the phases for implementing native DEB and RPM packaging using FPM in the `sysmqttd` release workflow.

## Phase 1: Packaging Scripts & Local Verification [checkpoint: ae9d4d6]

Build the installation and cleanup scripts and verify them.

- [x] Task: Create cross-platform post-install installer `deployment/post_install.sh`
- [x] Task: Create pre-uninstall cleanup hook `deployment/pre_uninstall.sh`
- [x] Task: Document local packaging manual verification process (FPM commands)
- [x] Task: Conductor - User Manual Verification 'Phase 1: Installer Script' (Protocol in workflow.md)

## Phase 2: GitHub Release Integration & Testing

Integrate the packaging logic into the GitHub Action release workflow.

- [x] Task: Modify `.github/workflows/release.yml` to install FPM and build `.deb`/`.rpm` packages for all matrix architectures
- [x] Task: Configure softprops/action-gh-release to upload the new assets and their checksums
- [x] Task: Verify the workflow parses cleanly
- [/] Task: Conductor - User Manual Verification 'Phase 2: Workflow Packaging' (Protocol in workflow.md)
