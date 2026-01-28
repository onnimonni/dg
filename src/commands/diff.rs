use crate::models::Graph;
use anyhow::{bail, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;

pub fn run(docs_dir: &str, base_ref: Option<&str>) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let base = base_ref.unwrap_or("HEAD");

    // Get the decisions directory relative path
    let decisions_dir = docs_path.join(".decisions");
    if !decisions_dir.exists() {
        bail!("No .decisions directory found");
    }

    // Get list of changed files from git
    let output = Command::new("git")
        .args([
            "diff",
            "--name-status",
            base,
            "--",
            decisions_dir.to_str().unwrap(),
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git diff failed: {}", stderr);
    }

    let diff_output = String::from_utf8_lossy(&output.stdout);

    // Also get untracked files
    let untracked_output = Command::new("git")
        .args([
            "ls-files",
            "--others",
            "--exclude-standard",
            decisions_dir.to_str().unwrap(),
        ])
        .output()?;

    let untracked = String::from_utf8_lossy(&untracked_output.stdout);

    // Parse changes
    let mut added: Vec<String> = Vec::new();
    let mut modified: Vec<String> = Vec::new();
    let mut deleted: Vec<String> = Vec::new();

    for line in diff_output.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(2, '\t').collect();
        if parts.len() != 2 {
            continue;
        }
        let status = parts[0];
        let file = parts[1];

        // Extract record ID from filename
        let record_id = extract_record_id(file);

        match status {
            "A" => added.push(record_id),
            "M" => modified.push(record_id),
            "D" => deleted.push(record_id),
            _ => {}
        }
    }

    // Add untracked files as "added"
    for line in untracked.lines() {
        if line.is_empty() || !line.ends_with(".md") {
            continue;
        }
        let record_id = extract_record_id(line);
        if !added.contains(&record_id) {
            added.push(record_id);
        }
    }

    // Load current graph to get titles
    let graph = Graph::load(docs_path).ok();

    // Print summary
    let total = added.len() + modified.len() + deleted.len();
    if total == 0 {
        println!("No changes since {}", base);
        return Ok(());
    }

    println!(
        "{} since {}:\n",
        format!(
            "{} record change{}",
            total,
            if total == 1 { "" } else { "s" }
        )
        .bold(),
        base.cyan()
    );

    if !added.is_empty() {
        println!("{}", "Added:".green().bold());
        for id in &added {
            let title = graph
                .as_ref()
                .and_then(|g| g.get(id))
                .map(|r| r.title().to_string())
                .unwrap_or_default();
            println!("  {} {}", format!("+{}", id).green(), title.dimmed());
        }
        println!();
    }

    if !modified.is_empty() {
        println!("{}", "Modified:".yellow().bold());
        for id in &modified {
            let title = graph
                .as_ref()
                .and_then(|g| g.get(id))
                .map(|r| r.title().to_string())
                .unwrap_or_default();
            println!("  {} {}", format!("~{}", id).yellow(), title.dimmed());
        }
        println!();
    }

    if !deleted.is_empty() {
        println!("{}", "Deleted:".red().bold());
        for id in &deleted {
            println!("  {}", format!("-{}", id).red());
        }
        println!();
    }

    Ok(())
}

fn extract_record_id(path: &str) -> String {
    // Extract ID from path like "docs/.decisions/DEC-001-some-title.md"
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| {
            // Take just the ID part (e.g., "DEC-001" from "DEC-001-some-title")
            let parts: Vec<&str> = s.splitn(3, '-').collect();
            if parts.len() >= 2 {
                format!("{}-{}", parts[0], parts[1])
            } else {
                s.to_string()
            }
        })
        .unwrap_or_else(|| path.to_string())
}
