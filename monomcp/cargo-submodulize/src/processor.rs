use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use toml_edit::{DocumentMut, value};
use cargo_metadata::Metadata;
use url::Url;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct CrateResponse {
    #[serde(rename = "crate")]
    pub crate_data: CrateData,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct CrateData {
    pub repository: Option<String>,
}

/// Represents information about a processed crate for reporting.
#[derive(Debug, Clone)]
pub struct ProcessedCrateInfo {
    pub name: String,
    pub version: Option<String>,
    pub repository: Option<String>,
    pub submodule_added: bool,
}

/// Processes a single Cargo.toml file, attempting to vendor dependencies as Git submodules.
/// It fetches repository information from cargo metadata or crates.io, adds submodules,
/// and updates the Cargo.toml to point to the local vendor path.
/// Returns a vector of `ProcessedCrateInfo` for reporting.
pub fn process_cargo_toml(
    cargo_toml_path: &String,
    dry_run: bool,
    client: &Client,
    metadata: &Metadata,
    cache: &mut HashMap<String, CrateResponse>,
) -> Result<Vec<ProcessedCrateInfo>> {
    let mut doc = std::fs::read_to_string(&cargo_toml_path)?.parse::<DocumentMut>()?;
    let mut processed_crates: Vec<ProcessedCrateInfo> = Vec::new();

    if let Some(deps) = doc["dependencies"].as_table_mut() {
        for (name, item) in deps.iter_mut() {
            println!("Processing dependency: {}", name);

            let version = super::utils::get_version(item);

            let mut repo_url: Option<String> = None;
            let mut submodule_added = false;

            // Try to get repository from cargo metadata
            if let Some(package) = metadata.packages.iter().find(|p| p.name == *name) {
                if let Some(source) = &package.source {
                    if source.repr.starts_with("git+") {
                        if let Ok(parsed_url) = Url::parse(&source.repr) {
                            // The URL might contain a commit hash or other query parameters.
                            // We want the base repository URL.
                            let mut base_url = parsed_url.clone();
                            base_url.set_query(None);
                            base_url.set_fragment(None);
                            repo_url = Some(base_url.to_string());
                            println!("Found Git repository via cargo metadata: {}", repo_url.as_ref().unwrap());
                        }
                    }
                }
            }

            // Fallback to crates.io API if not found via cargo metadata
            if repo_url.is_none() {
                if let Some(cached_response) = cache.get(&*name) {
                    if let Some(repo) = &cached_response.crate_data.repository {
                        repo_url = Some(repo.clone());
                        println!("Found repository via crates.io (cached): {}", repo_url.as_ref().unwrap());
                    }
                } else {
                    let url = format!("https://crates.io/api/v1/crates/{}", name);
                    let res = client.get(&url).header("User-Agent", "cargo-submodulize").send()?;

                    if res.status().is_success() {
                        let crate_response: CrateResponse = res.json()?;
                        if let Some(repo) = crate_response.crate_data.repository.clone() { // Clone here
                            repo_url = Some(repo);
                            println!("Found repository via crates.io: {}", repo_url.as_ref().unwrap());
                        }
                        cache.insert(name.to_string(), crate_response); // Insert into cache
                    } else {
                        eprintln!("Failed to fetch crate info for: {}. Status: {}", name, res.status());
                    }
                }
            }

            if let Some(repo) = repo_url.clone() { // Clone repo_url for ProcessedCrateInfo
                let vendor_path = format!("vendor/{}", name);

                if dry_run {
                    println!("[DRY RUN] Would add submodule for {} from {} to {}", name, repo, vendor_path);
                    if let Some(version) = version.clone() { // Clone version for dry_run message
                        println!("[DRY RUN] Would checkout tag {} for {}", version, name);
                    }
                    *item = value({
                        let mut new_item = toml_edit::InlineTable::new();
                        new_item.insert("path", toml_edit::Value::String(toml_edit::Formatted::new(vendor_path.clone())));
                        new_item
                    });
                    submodule_added = true; // In dry run, we assume it would be added
                } else {
                    let status = Command::new("git")
                        .arg("submodule")
                        .arg("add")
                        .arg(&repo)
                        .arg(&vendor_path)
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()?;

                    if status.success() {
                        println!("Added submodule for {}", name);

                        if let Some(version) = version.clone() { // Clone version for checkout
                            super::utils::checkout_tag(&*name, &vendor_path, &version)?;
                        }

                        *item = value({
                            let mut new_item = toml_edit::InlineTable::new();
                            new_item.insert("path", toml_edit::Value::String(toml_edit::Formatted::new(vendor_path.clone())));
                            new_item
                        });
                        submodule_added = true;
                    } else {
                        eprintln!("Failed to add submodule for {}. Git command exited with status: {}", name, status);
                    }
                }
            } else {
                eprintln!("Repository not found for crate: {}", name);
            }

            processed_crates.push(ProcessedCrateInfo {
                name: name.to_string(),
                version: version,
                repository: repo_url,
                submodule_added,
            });
        }
    }

    if dry_run {
        println!("[DRY RUN] Would write updated Cargo.toml to {}", cargo_toml_path);
    } else {
        std::fs::write(&cargo_toml_path, doc.to_string())?;
        println!("Successfully updated Cargo.toml: {}", cargo_toml_path);
    }

    Ok(processed_crates)
}
