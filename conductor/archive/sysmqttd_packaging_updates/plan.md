# Implementation Plan: Debian Section Label & ARMv6 Packaging Versioning

This plan outlines the steps required to configure package-level categorizations for Debian and RPM formats, and to adjust the automated GitHub Actions release workflow for the `linux-armv6` target.

## Phase 1: Packaging Metadata Setup [checkpoint: main]

Add section and group metadata to `Cargo.toml` to categorize the system daemon.

- [x] Task: Add `section = "utils"` under `[package.metadata.deb]` in `Cargo.toml`
- [x] Task: Add `group = "Applications/System"` under `[package.metadata.generate-rpm]` in `Cargo.toml`
- [x] Task: Local validation of `Cargo.toml` parsing

## Phase 2: Release Workflow Integration [checkpoint: main]

Integrate version-override logic into the GitHub Actions release workflow `.github/workflows/release.yml`.

- [x] Task: Update `.github/workflows/release.yml` Debian packaging step to append `+armv6` to version when `matrix.build == 'linux-armv6'` using `--deb-version`
- [x] Task: Update `.github/workflows/release.yml` RPM packaging step to append `+armv6` to version when `matrix.build == 'linux-armv6'` using `--set-metadata`
- [x] Task: Update target file naming and copy procedures within the Linux packaging block to ensure consistent outputs for the `+armv6` tagged files
- [x] Task: Validate YAML syntax of the modified `release.yml`
- [x] Task: Conductor - User Verification and dry-run packaging check
