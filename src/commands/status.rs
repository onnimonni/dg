use crate::models::{Graph, Status};
use anyhow::{anyhow, Result};
use chrono::Local;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, id: &str, new_status: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut graph = Graph::load(docs_path)?;

    let status = Status::from_str(new_status)
        .ok_or_else(|| anyhow!("Unknown status: {}", new_status))?;

    let record = graph
        .get_mut(id)
        .ok_or_else(|| anyhow!("Record not found: {}", id))?;

    let old_status = record.status().to_string();
    record.frontmatter.status = status;
    record.frontmatter.updated = Local::now().date_naive();
    record.save()?;

    println!(
        "{} {} status: {} -> {}",
        "Updated".green(),
        id.cyan(),
        old_status.yellow(),
        new_status.green()
    );

    // Update index
    let _ = graph.save_index();

    Ok(())
}
