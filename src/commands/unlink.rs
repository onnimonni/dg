use crate::models::Graph;
use anyhow::{anyhow, Result};
use chrono::Local;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, from: &str, link_type: &str, to: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut graph = Graph::load(docs_path)?;

    // Remove the link
    {
        let record = graph
            .get_mut(from)
            .ok_or_else(|| anyhow!("Source record not found: {}", from))?;
        let removed = record.frontmatter.links.remove_link(link_type, to)?;
        if !removed {
            return Err(anyhow!("Link not found: {} {} {}", from, link_type, to));
        }
        record.frontmatter.updated = Local::now().date_naive();
        record.save()?;
    }

    // Remove inverse link for certain types
    let inverse_type = match link_type {
        "supersedes" => Some("superseded_by"),
        "superseded_by" => Some("supersedes"),
        _ => None,
    };

    if let Some(inv) = inverse_type {
        if let Some(record) = graph.get_mut(to) {
            record.frontmatter.links.remove_link(inv, from)?;
            record.frontmatter.updated = Local::now().date_naive();
            record.save()?;
        }
    }

    println!(
        "{} {} {} {}",
        "Unlinked".yellow(),
        from.cyan(),
        link_type,
        to.cyan()
    );

    // Update index
    let _ = graph.save_index();

    Ok(())
}
