//! Finalize draft records by converting temporary IDs to permanent incremental IDs

use crate::commands::new::is_draft_id;
use crate::models::{Graph, Record};
use anyhow::{anyhow, Result};
use colored::Colorize;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Finalize all draft records, converting them to permanent IDs
pub fn run(docs_dir: &str, dry_run: bool) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let decisions_path = docs_path.join("decisions");

    if !decisions_path.exists() {
        return Err(anyhow!(
            "Decision graph not initialized. Run 'dg init' first."
        ));
    }

    let graph = Graph::load(docs_path)?;

    // Find all draft records
    let draft_records: Vec<&Record> = graph
        .all_records()
        .filter(|r| is_draft_id(r.id()))
        .collect();

    if draft_records.is_empty() {
        println!("{}", "No draft records found.".dimmed());
        return Ok(());
    }

    println!(
        "Found {} draft record(s):",
        draft_records.len().to_string().cyan()
    );

    // Group by record type and assign new IDs
    let mut id_mapping: HashMap<String, String> = HashMap::new();

    for record in &draft_records {
        let rt = record.record_type();
        let exclude: Vec<String> = id_mapping.values().cloned().collect();
        let new_id = graph.next_id_excluding(rt, &exclude);
        id_mapping.insert(record.id().to_string(), new_id.clone());

        if dry_run {
            println!(
                "  {} {} → {}",
                "Would rename:".yellow(),
                record.id().dimmed(),
                new_id.green()
            );
        } else {
            println!(
                "  {} {} → {}",
                "Renaming:".green(),
                record.id().dimmed(),
                new_id.green()
            );
        }
    }

    if dry_run {
        println!("\n{}", "Dry run - no changes made.".yellow());
        return Ok(());
    }

    // Apply renames
    for (old_id, new_id) in &id_mapping {
        rename_record(docs_path, old_id, new_id)?;
    }

    // Update all references in all documents
    update_all_references(docs_path, &id_mapping)?;

    // Reload and update index
    let graph = Graph::load(docs_path)?;
    let _ = graph.save_index();

    println!(
        "\n{} Finalized {} record(s)",
        "OK".green().bold(),
        id_mapping.len()
    );

    Ok(())
}

/// Rename a record file and update its ID in frontmatter
fn rename_record(docs_path: &Path, old_id: &str, new_id: &str) -> Result<()> {
    let decisions_path = docs_path.join("decisions");

    // Find the file with the old ID
    let old_file = find_record_file(&decisions_path, old_id)?;
    let content = fs::read_to_string(&old_file)?;

    // Update ID in frontmatter
    let id_pattern = Regex::new(&format!(r"^id:\s*{}", regex::escape(old_id)))?;
    let new_content = id_pattern.replace(&content, format!("id: {}", new_id));

    // Generate new filename
    let old_filename = old_file.file_name().unwrap().to_string_lossy();
    let new_filename = old_filename.replace(old_id, new_id);
    let new_file = decisions_path.join(&*new_filename);

    // Write new file
    fs::write(&new_file, new_content.as_ref())?;

    // Remove old file if different
    if old_file != new_file {
        fs::remove_file(&old_file)?;
    }

    Ok(())
}

/// Find the record file by ID
fn find_record_file(decisions_path: &Path, id: &str) -> Result<std::path::PathBuf> {
    for entry in fs::read_dir(decisions_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map(|e| e == "md").unwrap_or(false) {
            let filename = path.file_name().unwrap().to_string_lossy();
            if filename.starts_with(id) {
                return Ok(path);
            }
        }
    }
    Err(anyhow!("Record file not found for ID: {}", id))
}

/// Update all references to old IDs in all documents
fn update_all_references(docs_path: &Path, id_mapping: &HashMap<String, String>) -> Result<()> {
    let decisions_path = docs_path.join("decisions");

    for entry in fs::read_dir(&decisions_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map(|e| e == "md").unwrap_or(false) {
            let content = fs::read_to_string(&path)?;
            let mut new_content = content.clone();

            for (old_id, new_id) in id_mapping {
                new_content = new_content.replace(old_id, new_id);
            }

            if new_content != content {
                fs::write(&path, new_content)?;
            }
        }
    }

    Ok(())
}
