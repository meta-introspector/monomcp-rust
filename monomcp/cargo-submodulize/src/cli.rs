use clap::Parser;

/// `cargo-submodulize` is a tool to help manage Rust project dependencies by vendoring them
/// as Git submodules. It fetches repository information from crates.io or cargo metadata,
/// adds submodules, and updates Cargo.toml files to point to the local vendor path.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to a specific Cargo.toml file to process.
    #[arg(short, long)]
    pub path: Option<String>,
    /// Path to a file containing a list of Cargo.toml file paths to scan.
    #[arg(long)]
    pub scan_file_list: Option<String>,
    /// Perform a dry run without making any changes to the filesystem or Git repository.
    #[arg(long)]
    pub dry_run: bool,
    /// Recursively find all Cargo.toml files within the specified directory.
    #[arg(long)]
    pub recursive: bool,
    /// Path to a directory to scan for Cargo.toml files.
    #[arg(long)]
    pub dir: Option<String>,
    /// Stop processing after a specified number of errors.
    #[arg(long)]
    pub max_errors: Option<usize>,
}
