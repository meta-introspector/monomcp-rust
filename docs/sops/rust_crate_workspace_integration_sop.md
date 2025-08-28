# Standard Operating Procedure: Rust Crate Workspace Integration

## 1. Purpose
This SOP outlines the process for integrating external Rust crates into the `monomcp` workspace. The goal is to consolidate development, simplify dependency management, and enable a unified build and test environment for all components, ultimately leading to a more cohesive and maintainable codebase.

## 2. Scope
This SOP applies to any external Rust crate that needs to be developed or managed as part of the `monomcp` project, particularly those intended to become submodules or direct components of the monorepo.

## 3. Prerequisites
*   A local clone of the `monomcp` repository.
*   Familiarity with Rust's workspace and package management (`Cargo`).
*   Understanding of the external crate's structure and dependencies.

## 4. Procedure

### 4.1. Add the Crate as a Git Submodule (if applicable)
If the external crate is not already part of the `monomcp` repository, add it as a Git submodule. This ensures proper version control and integration.

```bash
git submodule add <repository_url> monomcp/<crate_name>
```

### 4.2. Modify the Crate's `Cargo.toml` (if necessary)
Depending on the crate's original structure, its `Cargo.toml` might need adjustments to fit into a workspace. Common modifications include:

*   **Removing redundant `[package]` sections:** If the crate is a simple library or binary that will inherit workspace-level metadata, some `[package]` fields might become redundant.
*   **Adjusting `path` dependencies:** If the crate has internal dependencies that are also being integrated into the workspace, their paths might need to be updated to be relative to the workspace root.
*   **Removing `[workspace]` sections:** If the external crate was itself a workspace, its `[workspace]` section should be removed as it will now be part of the `monomcp` super-workspace.

Navigate into the submodule directory and edit its `Cargo.toml`:

```bash
cd monomcp/<crate_name>
# Edit Cargo.toml using your preferred editor
```

### 4.3. Add the Crate to the `monomcp` Workspace
Edit the top-level `monomcp/Cargo.toml` file to include the new crate in the `[workspace.members]` section.

```toml
# monomcp/Cargo.toml
[workspace]
resolver = "2"
members = [
    # ... existing members ...
    "<crate_name>", # If the crate is directly in monomcp/
    "<path_to_crate>", # e.g., "vendor/<crate_name>" or "monomcp/<crate_name>"
]
```

### 4.4. Resolve Workspace Dependencies
Ensure that any dependencies shared between the new crate and other workspace members are correctly defined in the `[workspace.dependencies]` section of `monomcp/Cargo.toml`. This prevents duplicate dependencies and ensures consistent versions.

### 4.5. Verify Integration
After modifying `Cargo.toml` files, run `cargo check` or `cargo build` from the `monomcp` root directory to verify that the new crate integrates correctly and all dependencies are resolved.

```bash
cargo check
# or
cargo build
```
Address any compilation or dependency resolution errors.

### 4.6. Commit Changes
Commit the changes to both the submodule (if its `Cargo.toml` was modified) and the superproject (`monomcp/Cargo.toml`).

```bash
# Inside the submodule (if changes were made)
git add Cargo.toml
git commit -m "Refactor: Integrate into monomcp workspace"

# From the monomcp root
git add monomcp/<crate_name> # To update the submodule reference
git add monomcp/Cargo.toml
git commit -m "Feat: Add <crate_name> to workspace"
```

## 5. Troubleshooting
*   **Dependency Conflicts:** Use `cargo tree` to inspect the dependency graph and identify conflicting versions. Adjust `[workspace.dependencies]` or specific crate dependencies as needed.
*   **Path Errors:** Ensure all `path` dependencies are correctly specified relative to the workspace root.
*   **Build Failures:** Review error messages carefully. They often point to missing features, incorrect dependency declarations, or issues with the crate's code itself.

## 6. Automation (Future Work)
This manual process will serve as the basis for future automation efforts, potentially using a Rust-based MCP Git plugin to streamline the integration of Rust crates into the workspace.
