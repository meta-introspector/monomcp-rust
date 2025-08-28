# Standard Operating Procedure: Rust Crate Vendorization

## 1. Purpose
This SOP outlines the process for vendorizing Rust crates within the `monomcp` project. Vendorization involves copying a crate's source code into the project's repository and configuring Cargo to use this local copy instead of fetching it from `crates.io` or other registries. This is typically done to:
*   Apply local patches or bug fixes that are not yet upstream.
*   Ensure a fixed, immutable version of a dependency, independent of registry availability.
*   Reduce reliance on external network dependencies during builds.
*   Facilitate debugging or deeper integration with the crate's internals.

## 2. Scope
This SOP applies to any Rust crate that needs to be vendorized for the reasons mentioned above. It is particularly relevant for dependencies causing build issues, requiring custom modifications, or critical for offline builds.

## 3. Prerequisites
*   A local clone of the `monomcp` repository.
*   Familiarity with Rust's package management (`Cargo`) and dependency resolution.
*   Understanding of the crate to be vendorized and its dependencies.

## 4. Procedure

### 4.1. Identify the Crate and its Version
Determine the exact name and version of the crate to be vendorized. This can usually be found in `Cargo.lock` or the `Cargo.toml` of the direct dependent.

### 4.2. Create a `vendor/` Directory
If it doesn't already exist, create a `vendor/` directory at the root of your workspace (e.g., `monomcp/vendor/`). This directory will house all vendorized crates.

### 4.3. Download the Crate's Source
Download the source code for the specific version of the crate. The easiest way is often to let Cargo download it once, then copy it from the Cargo registry cache.

1.  **Temporarily add as a direct dependency (if not already):** Ensure the crate is listed in a `Cargo.toml` (e.g., `monomcp/Cargo.toml` or a relevant sub-crate's `Cargo.toml`).
2.  **Run `cargo fetch`:** This will download the crate to your Cargo registry cache.
    ```bash
cargo fetch
    ```
3.  **Locate and copy the source:** The source code will be in your Cargo home directory (usually `~/.cargo/registry/src/github.com-1ecc6299db9ec823/<crate_name>-<version>/`). Copy this directory to `monomcp/vendor/<crate_name>-<version>/`.

    ```bash
cp -r ~/.cargo/registry/src/github.com-1ecc6299db9ec823/<crate_name>-<version>/ monomcp/vendor/<crate_name>-<version>/
    ```

### 4.4. Configure `Cargo.toml` to Use the Vendorized Crate
Modify the top-level `monomcp/Cargo.toml` to tell Cargo to use the local vendorized copy. This is typically done using the `[patch.crates-io]` section.

```toml
# monomcp/Cargo.toml
[patch.crates-io]
<crate_name> = { path = "vendor/<crate_name>-<version>" }
```
Replace `<crate_name>` and `<version>` with the actual values.

### 4.5. Apply Necessary Patches (if any)
If the reason for vendorization is to apply a specific fix, navigate into the vendorized crate's directory (`monomcp/vendor/<crate_name>-<version>/`) and apply the changes directly to its source code.

### 4.6. Verify the Build
Run `cargo build` from the workspace root to ensure that Cargo correctly picks up the vendorized crate and that any applied patches resolve the original issue.

```bash
cargo build
```

### 4.7. Commit Changes
Commit the vendorized crate's source code and the modifications to `monomcp/Cargo.toml` to your Git repository.

```bash
git add monomcp/vendor/<crate_name>-<version>/
git add monomcp/Cargo.toml
git commit -m "Feat: Vendorize <crate_name> to apply custom patch"
```

## 5. Maintenance
*   **Updates:** When a new version of the upstream crate is released, you will need to manually update your vendorized copy and re-apply any custom patches.
*   **Dependency Tree:** Be mindful of the vendorized crate's own dependencies. If they also cause issues, they might need to be vendorized as well.

## 6. Automation (Future Work)
This manual process will serve as the basis for future automation efforts, potentially using a Rust-based MCP Git plugin to streamline the vendorization process.
