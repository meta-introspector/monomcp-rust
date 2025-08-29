# Monomcp Workspace Overview

This `monomcp` directory serves as the main workspace for the project, encapsulating various Rust crates that collectively form the core components of the Model Context Protocol (MCP) agent toolkit.

## Key Sub-projects/Crates

The `monomcp` workspace is organized into several functional groups:

*   **`agentic/`**: Contains crates related to agentic functionalities, including `agentic-core`, `agentic-tui` (Text User Interface), and `starlit-gui` (Graphical User Interface).
*   **`mcp-attr/`**: Houses attribute-related crates, such as `codegen` (code generation), `mcp-attr-macros` (procedural macros), and `mcp-attr` itself.
*   **`mcp-discovery/`**: Likely responsible for discovering MCP-related components or services.
*   **`memkb-mcp-rust/`**: Suggests memory-backed knowledge base functionalities for MCP.
*   **`MoCoPr/`**: (Model Context Protocol) - A core component, likely defining the protocol itself.
*   **`monomcp-core/`**: The foundational core utilities for the monomcp project.
*   **`paiml-mcp-agent-toolkit/server/`**: The server component of the paiml-mcp-agent-toolkit.
*   **`rmcp-agent/`**: A Rust-based MCP agent implementation.
*   **`rust-mcp-filesystem/`**: Handles filesystem interactions for MCP.
*   **`rust-mcp-sdk/`**: The Rust SDK for MCP, including `rust-mcp-macros`, `rust-mcp-sdk` itself, and `rust-mcp-transport`, along with various examples (`simple-mcp-client`, `hello-world-mcp-server`, etc.).
*   **`siumai/`**: (Purpose to be further defined, likely a specific utility or component).
*   **`turbomcp/`**: Suggests performance-oriented or accelerated MCP functionalities.

## Workspace Dependencies

Common and essential libraries are managed as workspace dependencies to ensure consistent versions and efficient compilation across all member crates. Key dependencies include:

*   **`serde` / `serde_json`**: For serialization and deserialization.
*   **`tokio`**: Asynchronous runtime for network and I/O operations.
*   **`anyhow` / `thiserror`**: For robust error handling.
*   **`uuid`**: For generating and working with UUIDs.
*   **`chrono`**: For date and time manipulation.
*   **`arrow-arith` (from `arrow-rs`)**: For Arrow-based data processing and arithmetic operations.
*   **`reqwest` / `tokio-tungstenite`**: For HTTP and WebSocket communication.
*   **`axum` / `tower` / `tower-http`**: For building web services and APIs.
*   **`clap`**: For command-line argument parsing.
*   **`tracing` / `tracing-subscriber`**: For structured logging and observability.
*   **`opentelemetry`**: For distributed tracing and metrics.

## Build Profiles

The `monomcp` workspace defines several build profiles to optimize for different development and deployment scenarios:

*   **`dev`**: Optimized for fast compilation and debugging (`opt-level = 1`, `debug = true`).
*   **`release`**: Optimized for maximum performance and binary size (`opt-level = 3`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, `strip = true`).
*   **`bench`**: Inherits from `release` but retains debug information (`debug = true`, `strip = false`) for benchmarking.
*   **`test`**: Optimized for testing (`opt-level = 1`, `debug = 2`).
*   **`dist`**: Inherits from `release` with `lto = "thin"`, likely for distribution-specific builds.

## Vendored Crates

The `vendor/` directory contains external repositories vendored directly into the monorepo. This ensures build reproducibility and allows for local modifications or specific version control. Key vendored repositories include:

*   **`cargo-rust-lang`**: A vendored version of the official Rust Cargo project, enabling deep integration and custom Cargo subcommands.
*   **`hugging-face-dataset-validator-rust`**: The `hf-dataset-validator` tool, crucial for code analysis and dataset generation.
*   **`arrow-rs`**: The Apache Arrow Rust implementation, providing columnar data processing capabilities and Parquet file format support.

## Quality Standards

This workspace adheres to the project's stringent Zero Tolerance Quality Standards, as outlined in `paiml-mcp-agent-toolkit/GEMINI.md`. This includes a focus on zero Self-Admitted Technical Debt (SATD), low code complexity, and comprehensive testing.