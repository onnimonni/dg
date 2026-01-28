use crate::models::Graph;
use anyhow::{anyhow, Result};
use chrono::Local;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, from: &str, link_type: &str, to: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut graph = Graph::load(docs_path)?;

    // Validate both records exist
    if graph.get(from).is_none() {
        return Err(anyhow!("Source record not found: {}", from));
    }
    if graph.get(to).is_none() {
        return Err(anyhow!("Target record not found: {}", to));
    }

    // Add the link
    {
        let record = graph
            .get_mut(from)
            .ok_or_else(|| anyhow!("Source record not found: {}", from))?;
        record.frontmatter.links.add_link(link_type, to)?;
        record.frontmatter.updated = Local::now().date_naive();
        record.save()?;
    }

    // Add inverse link for certain types
    let inverse_type = match link_type {
        "supersedes" => Some("superseded_by"),
        "superseded_by" => Some("supersedes"),
        _ => None,
    };

    if let Some(inv) = inverse_type {
        let record = graph
            .get_mut(to)
            .ok_or_else(|| anyhow!("Target record not found: {}", to))?;
        record.frontmatter.links.add_link(inv, from)?;
        record.frontmatter.updated = Local::now().date_naive();
        record.save()?;
        println!(
            "{} {} {} {} (+ inverse {} on {})",
            "Linked".green(),
            from.cyan(),
            link_type,
            to.cyan(),
            inv,
            to
        );
    } else {
        println!(
            "{} {} {} {}",
            "Linked".green(),
            from.cyan(),
            link_type,
            to.cyan()
        );
    }

    // Update index
    let _ = graph.save_index();

    Ok(())
}
