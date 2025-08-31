mod cli;
mod processor;
mod utils;
use anyhow::Result;
use std::collections::HashMap;
use reqwest::blocking::Client;
use clap::Parser;
use cli::Cli;
use processor::{process_cargo_toml, CrateResponse, ProcessedCrateInfo};
use std::fs;
use std::io::{self};

const CACHE_FILE: &str = "submodulize_cache.json";

/// Loads the cache from a JSON file.
fn load_cache() -> HashMap<String, CrateResponse> {
    match fs::read_to_string(CACHE_FILE) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|err| {
            eprintln!("Warning: Could not parse cache file {}: {}", CACHE_FILE, err);
            HashMap::new()
        }),
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            println!("Cache file {} not found, starting with empty cache.", CACHE_FILE);
            HashMap::new()
        },
        Err(err) => {
            eprintln!("Warning: Could not read cache file {}: {}", CACHE_FILE, err);
            HashMap::new()
        }
    }
}

/// Saves the cache to a JSON file.
fn save_cache(cache: &HashMap<String, CrateResponse>) -> Result<()> {
    let data = serde_json::to_string_pretty(cache)?;
    fs::write(CACHE_FILE, data)?;
    println!("Cache saved to {}.", CACHE_FILE);
    Ok(())
}

/// Main function for the `cargo-submodulize` tool.
/// It parses command-line arguments, loads a persistent cache,
/// processes Cargo.toml files to vendor dependencies as submodules,
/// saves the updated cache, and generates a report of processed crates.
fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = Client::new();
    let mut cache: HashMap<String, CrateResponse> = load_cache();
    let mut processed_crates_info: Vec<ProcessedCrateInfo> = Vec::new();
    let mut error_count = 0; // Initialize error count

    let cargo_toml_paths: Vec<String> = if let Some(scan_file_list_path) = cli.scan_file_list {
        std::fs::read_to_string(&scan_file_list_path)?
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    } else if let Some(dir_path) = cli.dir {
        if !cli.recursive {
            // Only look for Cargo.toml in the specified directory
            let cargo_toml_path = format!("{}/Cargo.toml", dir_path);
            if std::path::Path::new(&cargo_toml_path).exists() {
                vec![cargo_toml_path]
            } else {
                vec![]
            }
        } else {
            // Recursively find all Cargo.toml files
            let mut paths = Vec::new();
            for entry in walkdir::WalkDir::new(dir_path) {
                let entry = entry?;
                if entry.file_type().is_file() && entry.file_name() == "Cargo.toml" {
                    paths.push(entry.path().to_string_lossy().to_string());
                }
            }
            paths
        }
    } else {
        // Default to current directory's Cargo.toml
        vec!["Cargo.toml".to_string()]
    };

    for cargo_toml_path in cargo_toml_paths {
        println!("Processing Cargo.toml: {}", cargo_toml_path);
        let metadata = cargo_metadata::MetadataCommand::new().exec()?;
        let result = process_cargo_toml(&cargo_toml_path, cli.dry_run, &client, &metadata, &mut cache);
        match result {
            Ok(results) => {
                processed_crates_info.extend(results);
            },
            Err(e) => {
                eprintln!("Error processing {}: {}", cargo_toml_path, e);
                error_count += 1;
                if let Some(max_errors) = cli.max_errors {
                    if error_count >= max_errors {
                        eprintln!("Max errors ({}) reached. Stopping.", max_errors);
                        break;
                    }
                }
            }
        }
    }

    save_cache(&cache)?;

    // Generate report
    println!("\n--- Submodulize Report ---");
    if processed_crates_info.is_empty() {
        println!("No crates processed or no dependencies found.");
    } else {
        println!("{:<30} {:<15} {:<50} {:<15}", "Crate Name", "Version", "Repository", "Submodule Status");
        println!("{:-<30} {:-<15} {:-<50} {:-<15}", "", "", "", "");
        for info in processed_crates_info {
            println!("{:<30} {:<15} {:<50} {:<15}",
                     info.name,
                     info.version.unwrap_or_else(|| "N/A".to_string()),
                     info.repository.unwrap_or_else(|| "N/A".to_string()),
                     if info.submodule_added { "Added/Updated" } else { "Skipped" });
        }
    }

    Ok(())
}

