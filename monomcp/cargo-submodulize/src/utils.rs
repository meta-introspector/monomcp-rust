use anyhow::Result;
use toml_edit::Item;
use std::process::{Command, Stdio};

pub fn get_version(item: &Item) -> Option<String> {
    if let Some(s) = item.as_str() {
        return Some(s.to_string());
    } else if let Some(t) = item.as_inline_table() {
        if let Some(v) = t.get("version") {
            return v.as_str().map(|s| s.to_string());
        }
    }
    None
}

pub fn checkout_tag(name: &str, path: &str, version: &str) -> Result<()> {
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
