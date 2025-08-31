# Change Request: `cargo-submodulize` - Convert Dependencies to Git Submodules

*   **CRQ ID:** CRQ-CARGO-SUBMODULIZE-001
*   **Date:** 2025-08-31
*   **Status:** Draft
*   **Author:** Gemini CLI Agent

## 1. Objective

Automate the conversion of direct Rust project dependencies (specified in `Cargo.toml`) into Git submodules. This process will involve cloning the dependency's Git repository into a local `vendor/submodules/` directory (or a configurable path) and updating the `Cargo.toml` to reference these local submodule paths using `path` dependencies. The goal is to ensure the modified project remains fully compatible with unpatched `cargo` commands.

## 2. Scope

This CRQ covers the implementation of a new subcommand within `cargo-submodulize` (e.g., `cargo submodulize convert`) that will:

*   **Identify Dependencies:** Parse the project's `Cargo.toml` to identify direct dependencies that are sourced from Git repositories or implicitly from `crates.io` (for which a Git source can be inferred/configured).
*   **Clone and Add Submodules:** For each identified dependency, clone its Git repository into a structured directory within `vendor/submodules/` (e.g., `vendor/submodules/dependency-name`) and add it as a Git submodule to the main project.
*   **Modify `Cargo.toml`:** Update the corresponding entries in `Cargo.toml` to use `path` dependencies, pointing to the newly created local submodule directories (e.g., `dependency = { path = "vendor/submodules/dependency-name" }`).
*   **Handle `Cargo.lock`:** The tool should gracefully handle an existing `Cargo.lock` file, allowing `cargo` to update it naturally after the `Cargo.toml` modifications.
*   **Error Handling:** Provide clear error messages for scenarios such as:
    *   Dependency not being a Git repository.
    *   Failure to clone a repository.
    *   Existing submodule with the same name.

## 3. Motivation / Rationale

*   **Offline Builds:** Enable building projects without requiring continuous internet access for fetching dependencies from `crates.io` or remote Git repositories.
*   **Reproducibility & Auditability:** Enhance build reproducibility by pinning specific versions of dependencies via Git commits within submodules, making the entire dependency tree auditable.
*   **Local Development & Debugging:** Facilitate easier local modification, patching, and debugging of dependencies by having their source code directly within the project's repository.
*   **Monorepo Alignment:** Support monorepo strategies where dependencies might be managed as internal sub-projects or shared components.
*   **Hermetic Builds:** Contribute to more hermetic build environments by controlling the exact source of all dependencies.

## 4. Proposed Changes / Technical Approach

### 4.1. New Subcommand

A new subcommand, `convert`, will be added to `cargo-submodulize`:

```bash
cargo submodulize convert [OPTIONS]
```

*   **Options:**
    *   `--path <DIR>`: Optional. Specifies the base directory for submodules (default: `vendor/submodules/`).
    *   `--dependencies <NAMES>`: Optional. Comma-separated list of specific dependencies to convert. If omitted, all direct Git-sourced dependencies will be converted.
    *   `--force`: Optional. Overwrite existing submodule directories if they exist.

### 4.2. Implementation Details

*   **`Cargo.toml` Parsing:** Utilize the `cargo_toml` crate to programmatically read and modify `Cargo.toml` files.
*   **Git Operations:** Leverage the `git2` crate for Git repository cloning and submodule management.
*   **Dependency Resolution Logic:**
    *   Iterate through `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`, and `[target.<triple>.dependencies]` sections.
    *   For each dependency, determine its Git source URL. If a `crates.io` dependency, a mechanism (e.g., a configurable mapping or a web lookup) might be needed to find its canonical Git repository. Initially, focus on explicitly Git-sourced dependencies.
    *   If a dependency is already a `path` dependency, it will be skipped.
*   **Submodule Creation:**
    *   For each selected dependency, execute `git submodule add <repo_url> <path_to_submodule>`.
    *   The submodule path will be constructed as `<base_path>/<dependency_name>`.
*   **`Cargo.toml` Modification:**
    *   For each converted dependency, update its entry in `Cargo.toml` to:
        ```toml
        [dependencies]
        dependency-name = { path = "vendor/submodules/dependency-name" }
        ```
    *   Preserve other fields like `features`, `optional`, etc., if they exist.
*   **Output:** Provide clear console output indicating which dependencies were converted, skipped, or failed.

## 5. Impact

*   **Repository Size:** The project's Git repository size will increase due to the inclusion of dependency source code as submodules.
*   **Build Process:** Initial setup will require `git submodule update --init --recursive` for fresh clones of the main repository. Subsequent `cargo build` commands will use local sources.
*   **`Cargo.toml` and `.gitmodules`:** These files will be modified by the tool.
*   **CI/CD:** CI/CD pipelines will need to be updated to include `git submodule update --init --recursive` during the clone/checkout phase.

## 6. Verification / Testing

*   **Unit Tests:**
    *   Test `Cargo.toml` parsing and modification logic with various dependency formats (version, git, path).
    *   Test path construction for submodules.
*   **Integration Tests:**
    *   Create a dedicated test project with various types of dependencies (Git, `crates.io`).
    *   Run `cargo submodulize convert` on this test project.
    *   Verify the contents of `.gitmodules` after execution.
    *   Verify the changes in `Cargo.toml` for converted dependencies.
    *   Run `cargo build` and `cargo test` on the modified test project to ensure it compiles and runs successfully, confirming compatibility with unpatched `cargo`.
    *   Test error handling scenarios (e.g., non-existent repo, network issues).

## 7. Future Considerations

*   **Revert Functionality:** Consider adding a subcommand to revert the changes (remove submodules and restore `Cargo.toml` entries).
*   **Transitive Dependencies:** Explore options to recursively convert transitive dependencies into submodules.
*   **`crates.io` to Git Mapping:** Implement a robust mechanism to map `crates.io` dependencies to their canonical Git repositories, possibly through a configurable lookup table or by querying `crates.io` API.
*   **Hybrid Approach:** Investigate integration with `cargo vendor` for a hybrid strategy where some dependencies are submodulized and others are simply vendored.
*   **Pre-commit Hook Integration:** Provide guidance or a utility for integrating this process into pre-commit hooks.