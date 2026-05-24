# Rust Style Guide: `sysmqttd`

This style guide defines coding standards, idioms, safety constraints, and formatting rules for the `sysmqttd` Rust codebase.

## 1. Code Formatting
*   Always format code using `cargo fmt` with default rustfmt rules.
*   Line length limit: 100 characters.

## 2. Idiomatic Rust
*   Use standard compiler lints to guide quality. Avoid `unsafe` block usage unless absolutely required by a hardware dependency (which should not be needed here).
*   Prefer structural error handling using `thiserror` or custom `Result`/`Error` enums instead of `.unwrap()` or `.expect()`.
*   Handle optionals safely using `if let`, `let else`, or `map`/`and_then` pattern.

## 3. Asynchronous Programming
*   Keep the main tokio scheduler light. Since this runs on a single-core CPU (ARMv6), avoid heavy task spawning.
*   Ensure all IO blocks are asynchronous or offloaded to `tokio::task::spawn_blocking` (though for our telemetry reads, they are simple file reads and can be done synchronously inside the 60-second loop without impacting the system since the loop sleeps most of the time).
*   Handle tokio cancellation tokens or signals gracefully to allow clean shutdowns.

## 4. Size & Resource Management
*   **Static allocations:** Avoid unnecessary dynamic allocations (e.g. `String`, `Vec`) inside the 60-second telemetry loop. Reuse buffers if possible.
*   **Minimal cloning:** Pass parameters by reference (`&str`, `&path`) instead of cloning values.
*   **Minimized crates:** Do not pull in large transitive dependency trees. Ensure `sysinfo` is compiled with minimal features.

## 5. Documentation
*   Document all public structs, enums, and functions using Rustdoc triple-slash comments (`///`).
*   Include a short summary section for each module explaining its role.
