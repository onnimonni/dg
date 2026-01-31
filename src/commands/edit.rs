use crate::models::Graph;
use anyhow::{bail, Result};
use std::path::Path;
use std::process::Command;

pub fn run(docs_dir: &str, id: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let record = match graph.get(id) {
        Some(r) => r,
        None => bail!("Record not found: {}", id),
    };

    let file_path = &record.path;

    // Get editor from environment
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| {
            // Try to find a sensible default
            if Command::new("which")
                .arg("nvim")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                "nvim".to_string()
            } else if Command::new("which")
                .arg("vim")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                "vim".to_string()
            } else if Command::new("which")
                .arg("nano")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                "nano".to_string()
            } else {
                "vi".to_string()
            }
        });

    // Get file modification time before editing
    let before_mtime = std::fs::metadata(file_path)?.modified()?;

    // Open editor
    let status = Command::new(&editor).arg(file_path).status()?;

    if !status.success() {
        bail!("Editor exited with error");
    }

    // Check if file was modified
    let after_mtime = std::fs::metadata(file_path)?.modified()?;

    if after_mtime != before_mtime {
        // Reindex to pick up changes
        println!("File modified, reindexing...");
        crate::commands::reindex::run(docs_dir)?;
    } else {
        println!("No changes made.");
    }

    Ok(())
}
