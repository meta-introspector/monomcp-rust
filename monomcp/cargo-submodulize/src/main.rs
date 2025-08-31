
use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::process::{Command, Stdio};
use toml_edit::{DocumentMut, value, Item};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    path: Option<String>,
}

#[derive(Deserialize, Debug)]
struct CrateResponse {
    #[serde(rename = "crate")]
    crate_data: CrateData,
}

#[derive(Deserialize, Debug)]
struct CrateData {
    repository: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let cargo_toml_path = cli.path.unwrap_or_else(|| "Cargo.toml".to_string());
    let mut doc = std::fs::read_to_string(&cargo_toml_path)?.parse::<DocumentMut>()?;

    let client = Client::new();

    if let Some(deps) = doc["dependencies"].as_table_mut() {
        for (name, item) in deps.iter_mut() {
            println!("Processing dependency: {}", name);

            let version = get_version(item);

            let url = format!("https://crates.io/api/v1/crates/{}", name);
            let res = client.get(&url).header("User-Agent", "cargo-submodulize").send()?;
            
            if res.status().is_success() {
                let crate_response: CrateResponse = res.json()?;
                if let Some(repo) = crate_response.crate_data.repository {
                    println!("Found repository: {}", repo);

                    let vendor_path = format!("vendor/{}", name);
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

                        if let Some(version) = version {
                            checkout_tag(name, &vendor_path, &version)?;
                        }

                        *item = value({
                            let mut new_item = toml_edit::InlineTable::new();
                            new_item.insert("path", toml_edit::Value::String(toml_edit::Formatted::new(vendor_path.clone())));
                            new_item
                        });
                    } else {
                        eprintln!("Failed to add submodule for {}. Git command exited with status: {}", name, status);
                    }
                } else {
                    eprintln!("Repository not found for crate: {}", name);
                }
            } else {
                eprintln!("Failed to fetch crate info for: {}. Status: {}", name, res.status());
            }
        }
    }

    std::fs::write(&cargo_toml_path, doc.to_string())?;
    println!("Successfully updated Cargo.toml");

    Ok(())
}

fn get_version(item: &Item) -> Option<String> {
    if let Some(s) = item.as_str() {
        return Some(s.to_string());
    } else if let Some(t) = item.as_inline_table() {
        if let Some(v) = t.get("version") {
            return v.as_str().map(|s| s.to_string());
        }
    }
    None
}

fn checkout_tag(name: &str, path: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tags_output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("tag")
        .arg("-l")
        .output()?;

    let tags = String::from_utf8(tags_output.stdout)?;
    let mut found_tag = None;

    let possible_tags = [
        format!("v{}", version),
        version.to_string(),
        format!("{}-{}", name, version),
    ];

    for tag in tags.lines() {
        if possible_tags.contains(&tag.to_string()) {
            found_tag = Some(tag.to_string());
            break;
        }
    }

    if let Some(tag) = found_tag {
        println!("Found tag {} for version {}, checking out.", tag, version);
        let status = Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("checkout")
            .arg(&tag)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        if !status.success() {
            eprintln!("Failed to checkout tag {} for {}.", tag, name);
        }
    } else {
        eprintln!("Could not find a matching tag for version {} of {}. Please check the repository's tags.", version, name);
    }

    Ok(())
}
