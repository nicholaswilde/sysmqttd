# Specification: Version and Help CLI Arguments for `sysmqttd`

This specification defines the requirements for adding command line arguments to `sysmqttd` to query version information and display help screens.

## Overview
To improve usability and follow standard command line conventions, `sysmqttd` will support `-v`/`--version` and `-h`/`--help` flags. Since this binary must remain extremely lightweight (retaining its footprint limits and small binary size), we will implement command-line argument scanning directly using `std::env::args()` without introducing external dependency crates like `clap` or `argh`.

## Functional Requirements
1. **Help Flag (`-h` or `--help`)**:
   - When executed with `-h` or `--help`, the binary must print a clear usage instructions menu to standard output (`stdout`) and exit cleanly with exit code `0`.
   - The help output must explain the daemon description, available options (`-h, --help` and `-v, --version`), and expected configuration environment variables.

2. **Version Flag (`-v` or `--version`)**:
   - When executed with `-v` or `--version`, the binary must print the current version (e.g. `sysmqttd v0.1.0`) to `stdout` and exit cleanly with exit code `0`.
   - The version printed must be dynamically pulled at compile-time using Rust's `env!("CARGO_PKG_VERSION")` macro.

3. **Execution Precedence & Exclusivity**:
   - If either flag is passed, the program must print the requested information and exit immediately without checking, parsing, or complaining about missing configuration or connection environment variables, and without initiating the tokio async runtime daemon connection loop.
   - If an invalid argument is provided (e.g., `--unknown`), it must print a brief error message (e.g., `Error: Unknown argument '--unknown'. Use --help for usage details.`) to standard error (`stderr`) and exit with exit code `1`.

## Acceptance Criteria
- Running `./sysmqttd --help` or `./sysmqttd -h` prints standard usage details and exits with code `0`.
- Running `./sysmqttd --version` or `./sysmqttd -v` prints `sysmqttd v<version>` and exits with code `0`.
- Running `./sysmqttd --unknown` prints an error message to `stderr` and exits with code `1`.
- Displaying help or version details must NOT require any environment variables to be set.
- Binary size remains optimized and no new CLI parsing crates are added to `Cargo.toml`.
- All automated unit tests and code coverage targets (>90%) are fully retained.
