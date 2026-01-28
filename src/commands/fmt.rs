use crate::models::Record;
use anyhow::{anyhow, Result};
use colored::Colorize;
use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn run(docs_dir: &str, check: bool, files: Option<Vec<String>>, quiet: bool) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let decisions_dir = docs_path.join(".decisions");

    let mut total = 0;
    let mut formatted = 0;
    let mut failed = 0;

    let paths: Vec<_> = if let Some(file_list) = files {
        file_list
            .into_iter()
            .map(|f| Path::new(&f).to_path_buf())
            .collect()
    } else {
        WalkDir::new(&decisions_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
            .map(|e| e.path().to_path_buf())
            .collect()
    };

    for path in paths {
        if !path.exists() {
            eprintln!("{} File not found: {}", "ERROR".red(), path.display());
            failed += 1;
            continue;
        }

        total += 1;

        match format_file(&path, check) {
            Ok(was_formatted) => {
                if was_formatted {
                    formatted += 1;
                    if !quiet {
                        if check {
                            println!("{} {}", "NEEDS FMT".yellow(), path.display());
                        } else {
                            println!("{} {}", "Formatted".green(), path.display());
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{} {}: {}", "ERROR".red(), path.display(), e);
                failed += 1;
            }
        }
    }

    if check {
        if formatted > 0 || failed > 0 {
            if !quiet {
                println!();
                println!(
                    "{} {} files need formatting, {} errors",
                    "FAIL".red().bold(),
                    formatted,
                    failed
                );
            }
            std::process::exit(1);
        } else if !quiet {
            println!();
            println!(
                "{} All {} files are properly formatted",
                "OK".green().bold(),
                total
            );
        }
    } else if !quiet {
        println!();
        println!(
            "{} Formatted {}/{} files ({} errors)",
            "Done".green().bold(),
            formatted,
            total,
            failed
        );
    }

    Ok(())
}

fn format_file(path: &Path, check_only: bool) -> Result<bool> {
    let content = fs::read_to_string(path)?;
    let formatted = format_markdown(&content)?;

    if content == formatted {
        return Ok(false);
    }

    if !check_only {
        fs::write(path, &formatted)?;
    }

    Ok(true)
}

fn format_markdown(content: &str) -> Result<String> {
    // Parse frontmatter and body
    let re = Regex::new(r"(?s)^---\n(.*?)\n---\n(.*)$")?;
    let caps = re
        .captures(content)
        .ok_or_else(|| anyhow!("Invalid frontmatter format"))?;

    // Safe: regex pattern guarantees groups 1 and 2 exist when captures succeeds
    let yaml_str = caps
        .get(1)
        .ok_or_else(|| anyhow!("Missing YAML frontmatter"))?
        .as_str();
    let body = caps
        .get(2)
        .ok_or_else(|| anyhow!("Missing content body"))?
        .as_str();

    // Parse and reserialize YAML for consistent formatting
    let record: Result<serde_yaml::Value, _> = serde_yaml::from_str(yaml_str);
    let formatted_yaml = match record {
        Ok(value) => format_yaml(&value)?,
        Err(_) => yaml_str.to_string(), // Keep original if parse fails
    };

    // Format body
    let formatted_body = format_body(body);

    Ok(format!("---\n{}\n---\n{}", formatted_yaml, formatted_body))
}

fn format_yaml(value: &serde_yaml::Value) -> Result<String> {
    // Define the desired key order for frontmatter
    let key_order = [
        "type", "id", "title", "status", "created", "updated", "authors", "tags", "links",
    ];

    if let serde_yaml::Value::Mapping(map) = value {
        let mut lines = Vec::new();

        // First, output keys in order
        for key in &key_order {
            if let Some(val) = map.get(&serde_yaml::Value::String(key.to_string())) {
                lines.push(format_yaml_field(key, val));
            }
        }

        // Then output any extra keys not in our order
        for (k, v) in map {
            if let serde_yaml::Value::String(key) = k {
                if !key_order.contains(&key.as_str()) {
                    lines.push(format_yaml_field(key, v));
                }
            }
        }

        Ok(lines.join("\n"))
    } else {
        Ok(serde_yaml::to_string(value)?.trim_end().to_string())
    }
}

fn format_yaml_field(key: &str, value: &serde_yaml::Value) -> String {
    match value {
        serde_yaml::Value::Sequence(seq) if seq.is_empty() => {
            format!("{}: []", key)
        }
        serde_yaml::Value::Sequence(seq) => {
            // Sort arrays of strings (tags, authors)
            if key == "tags" || key == "authors" {
                let mut items: Vec<String> = seq
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                items.sort();
                format!("{}: [{}]", key, items.join(", "))
            } else {
                // For other sequences, use inline format if all are simple strings
                let all_simple = seq.iter().all(|v| v.as_str().is_some());
                if all_simple && seq.len() <= 5 {
                    let items: Vec<String> = seq
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    format!("{}: [{}]", key, items.join(", "))
                } else {
                    let yaml = serde_yaml::to_string(&serde_yaml::Value::Mapping({
                        let mut m = serde_yaml::Mapping::new();
                        m.insert(serde_yaml::Value::String(key.to_string()), value.clone());
                        m
                    }))
                    .unwrap_or_default();
                    yaml.trim_end().to_string()
                }
            }
        }
        serde_yaml::Value::Mapping(map) => {
            // For nested mappings like 'links', format each on its own line
            let mut lines = vec![format!("{}:", key)];
            for (k, v) in map {
                if let serde_yaml::Value::String(subkey) = k {
                    let formatted = format_yaml_field(subkey, v);
                    for line in formatted.lines() {
                        lines.push(format!("  {}", line));
                    }
                }
            }
            lines.join("\n")
        }
        serde_yaml::Value::String(s) => {
            // Quote strings that contain special characters
            if s.contains(':') || s.contains('#') || s.contains('"') || s.contains('\n') {
                format!("{}: \"{}\"", key, s.replace('"', "\\\""))
            } else {
                format!("{}: {}", key, s)
            }
        }
        serde_yaml::Value::Null => format!("{}: null", key),
        _ => {
            let yaml = serde_yaml::to_string(value).unwrap_or_default();
            format!("{}: {}", key, yaml.trim())
        }
    }
}

fn format_body(body: &str) -> String {
    let mut lines: Vec<String> = body.lines().map(|l| l.trim_end().to_string()).collect();

    // Ensure blank line after frontmatter separator
    if !lines.is_empty() && !lines[0].is_empty() {
        lines.insert(0, String::new());
    }

    // Ensure single blank line before headings
    let mut result = Vec::new();
    let mut prev_empty = true;

    for (i, line) in lines.iter().enumerate() {
        let is_heading = line.starts_with('#');
        let is_empty = line.is_empty();

        if is_heading && !prev_empty && i > 0 {
            result.push(String::new());
        }

        // Don't add multiple consecutive empty lines
        if is_empty && prev_empty && !result.is_empty() {
            continue;
        }

        result.push(line.clone());
        prev_empty = is_empty;
    }

    // Ensure single newline at end
    while result.last().map_or(false, |l| l.is_empty()) {
        result.pop();
    }
    result.push(String::new());

    result.join("\n")
}

/// Format a record in-memory and return the formatted content
#[allow(dead_code)]
pub fn format_record(record: &Record) -> Result<String> {
    let yaml = serde_yaml::to_string(&record.frontmatter)?;
    let content = format!("---\n{}---\n{}", yaml, record.content);
    format_markdown(&content)
}
