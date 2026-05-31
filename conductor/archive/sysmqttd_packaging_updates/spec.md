# Specification: Debian Section Label & ARMv6 Packaging Versioning

This specification defines the packaging requirements and adjustments for `sysmqttd` releases to support clean package index integration in repositories like `reprepro`.

## Overview
When hosting packages in a Debian repository (such as `reprepro`), packages require specific metadata fields, and file names must be unique per architecture/version combination. 

This track addresses three packaging enhancements:
1. **Debian Section Category**: Add a `section` category label to the Debian package metadata.
2. **RPM Category (Group)**: Determine and implement the equivalent category metadata for RPM packages to maintain package parity and metadata completeness.
3. **ARMv6 Build Versioning Distinction**: Customize the version string for the ARMv6 architecture build (`linux-armv6` matrix target) to append `+armv6` to the package version (e.g., `0.1.19+armv6`). This prevents filename conflicts in repositories that differentiate ARMv6 and ARMv7 packages while sharing the same `armhf` Debian architecture label.

---

## Technical Analysis & Requirements

### 1. Debian Section Category
- **Requirement**: Add `section = "utils"` (or similar standard Debian section like `admin` or `net`) under the `[package.metadata.deb]` table in `Cargo.toml`.
- **Reasoning**: Without a `Section` field in the Debian control file, some repository managers (like `reprepro`) fail or throw warnings during ingestion. The `utils` section matches the nature of `sysmqttd` as a lightweight system monitoring utility.

### 2. RPM Category Label
- **Requirement**: Set the `group` metadata field under the `[package.metadata.generate-rpm]` table in `Cargo.toml` to `Applications/System` (or similar standard RPM category like `Development/Tools`).
- **Reasoning**: Traditional RPM specifications utilize the `Group` tag for package categorization. Adding `group = "Applications/System"` ensures package metadata richness and backward compatibility with standard RPM repositories.

### 3. ARMv6 Packaging Version Override in `release.yml`
- **Requirement**: When building for the `linux-armv6` target, the workflow must append `+armv6` to the version used during package generation.
- **Workflow Implementation**:
  - The workflow currently builds packages using:
    ```bash
    cargo deb --no-build --target "${{ matrix.target }}"
    cargo generate-rpm --target "${{ matrix.target }}" --auto-req disabled
    ```
  - For the `linux-armv6` target (matrix target: `arm-unknown-linux-gnueabihf`), we override the version string:
    - For `cargo-deb`: Use the `--deb-version "${version}+armv6"` CLI argument.
    - For `cargo-generate-rpm`: Use the `--set-metadata 'version = "'"${version}+armv6"'"'` CLI argument.
  - The generated assets (filenames and SHA256 checksum files) must incorporate the updated `+armv6` version string, producing filenames like `sysmqttd-0.1.19+armv6-arm-unknown-linux-gnueabihf.deb` and `sysmqttd-0.1.19+armv6-arm-unknown-linux-gnueabihf.rpm`.

---

## Acceptance Criteria
- **Debian Control Info**: Building the Debian package includes a `Section: utils` field in the generated control file.
- **RPM Spec Info**: Building the RPM package contains the `Group: Applications/System` tag.
- **ARMv6 Build Versioning**: The `linux-armv6` release assets include the `+armv6` suffix in both the `.deb` and `.rpm` file names and internal metadata versions, separating them from the `linux-armv7` assets.
- **Other Targets (x86_64, armv7, aarch64)**: Retain their normal untagged version string (e.g. `0.1.19`).
- **GitHub Release Upload**: The custom-versioned armv6 deb/rpm files and their sha256 checksum files upload successfully to the draft release.
