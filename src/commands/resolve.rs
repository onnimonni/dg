use crate::models::{Graph, RecordType, Status};
use anyhow::{bail, Result};
use chrono::Local;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, id: &str, note: Option<&str>) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut graph = Graph::load(docs_path)?;

    let record = match graph.get_mut(id) {
        Some(r) => r,
        None => bail!("Record not found: {}", id),
    };

    // Check it's an incident
    if record.record_type() != &RecordType::Incident {
        bail!(
            "{} is a {} not an incident. Use `dg status {} resolved` instead.",
            id,
            record.record_type(),
            id
        );
    }

    // Check it's not already resolved
    if record.status() == &Status::Resolved {
        println!("{} {} is already resolved", "✓".green(), id);
        return Ok(());
    }

    // Update status
    record.frontmatter.status = Status::Resolved;

    // Update the updated date
    let today = Local::now().date_naive();
    record.frontmatter.updated = today;

    // Add resolution note if provided
    if let Some(note_text) = note {
        // Append to content
        let resolution_section = format!(
            "\n\n## Resolution\n\n{}\n\n*Resolved: {}*\n",
            note_text,
            today.format("%Y-%m-%d")
        );
        record.content.push_str(&resolution_section);
    }

    // Save the record
    record.save()?;

    println!(
        "{} Resolved {} - {}",
        "✓".green(),
        id.cyan(),
        record.title()
    );

    if note.is_some() {
        println!("  Added resolution note");
    }

    // Reindex
    crate::commands::reindex::run(docs_dir)?;

    Ok(())
}
