# /dependency-update

Automates updating cargo dependencies, verifying compatibility, and committing changes.

## Description
This skill ensures all cargo dependencies are kept up-to-date with their latest compatible versions, verifies that the updates do not introduce any compilation or test regressions, checks code quality and formatting, and commits/pushes the updates cleanly.

## Protocol

1. **Update Cargo Dependencies:**
   - Run `cargo update` to update dependencies in `Cargo.lock` to their latest compatible versions.

2. **Verify Build and Compatibility:**
   - Run `cargo check` to guarantee the codebase compiles without warnings or errors.
   - Run `cargo check --tests` to guarantee the test suites compile cleanly.

3. **Verify Project Quality & Verification Gates:**
   - Run `task fmt` (or `cargo fmt --all -- --check`) to verify code formatting compliance.
   - Run `task lint` to verify Clippy lints are fully clean.
   - Run `task test` to execute the full unit and integration test suites, guaranteeing all 51 tests pass.
   - Run `task coverage` to verify that code coverage remains above the strict 90% line gate.

4. **Staging and Storing Changes:**
   - Check `git status --porcelain` to identify updated lock files.
   - Stage the updated `Cargo.lock` and any related files.
   - Commit the changes with the exact message: `chore: Update cargo dependencies`.

5. **Atomic Push:**
   - Push the clean commits atomically to the remote repository:
     `git push origin main`

6. **Report:**
   - Provide a concise summary of the updated dependencies, test passes, coverage compliance, and push status.
