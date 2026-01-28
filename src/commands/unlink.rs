use crate::models::Graph;
use anyhow::Result;
use chrono::Local;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, from: &str, link_type: &str, to: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut graph = Graph::load(docs_path)?;

    // Remove the link
    {
        let record = match graph.get_mut(from) {
            Some(r) => r,
            None => {
                eprintln!(
                    "{} Source record not found: {}",
                    "Error:".red().bold(),
                    from.cyan()
                );
                std::process::exit(1);
            }
        };
        let removed = record.frontmatter.links.remove_link(link_type, to)?;
        if !removed {
            eprintln!(
                "{} Link not found: {} {} {}",
                "Error:".red().bold(),
                from.cyan(),
                link_type,
                to.cyan()
            );
            std::process::exit(1);
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
