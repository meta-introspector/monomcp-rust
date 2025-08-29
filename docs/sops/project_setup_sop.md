# Standard Operating Procedure: Project Setup

## 1. Introduction

This document outlines the standard procedure for setting up the `monomcp` project on a local development environment. Following these steps will ensure a consistent and functional setup, allowing users to contribute to and utilize the project's tools.

## 2. Prerequisites

Before proceeding, ensure you have the following installed:

*   **Git:** Version control system.
*   **Rustup:** The Rust toolchain installer and manager. This will allow you to install the Rust compiler (`rustc`) and Cargo (`cargo`).

## 3. Procedure

### 3.1. Clone the Repository

Begin by cloning the `monomcp` repository from GitHub:

```bash
git clone https://github.com/meta-introspector/monomcp.git
cd monomcp
```

### 3.2. Initialize Submodules

This project utilizes Git submodules for managing external dependencies (like `arrow-rs` and `cargo`). After cloning, you must initialize and update these submodules:

```bash
git submodule update --init --recursive
```

### 3.3. Build the Project

Navigate into the `monomcp` workspace directory and build the entire project. This will compile all workspace members and their dependencies.

```bash
cd monomcp
cargo build --workspace
```

*   **Note:** The initial build may take some time as Cargo downloads and compiles all dependencies. You might see warnings related to unused patches; these are expected and do not prevent the build from succeeding.

### 3.4. Verify Setup

After a successful build, you can verify the setup by running the `project_reporter` tool:

```bash
cd .. # Go back to the monomcp root directory
cargo run --manifest-path project_reporter/Cargo.toml
```

This should output a table of project metadata for the analyzed crates.

## 4. Troubleshooting

*   **"multiple workspace roots found" error:** This indicates an issue with nested workspaces. Ensure you are running `cargo build --workspace` from the `monomcp` directory, and that no submodules are incorrectly added as direct workspace members in `monomcp/Cargo.toml`.
*   **Compilation Errors:** If you encounter compilation errors, ensure your Rust toolchain is up-to-date (`rustup update`) and that all submodules are correctly initialized (`git submodule update --init --recursive`). Refer to the project's `Cargo.toml` files and `[patch.crates-io]` sections for specific dependency versions.

---
