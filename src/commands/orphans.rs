use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let orphans: Vec<_> = graph
        .all_records()
        .filter(|r| {
            let has_outgoing = !r.frontmatter.links.all_links().is_empty();
            let has_incoming = !graph.incoming_edges(r.id()).is_empty();
            !has_outgoing && !has_incoming
        })
        .collect();

    if orphans.is_empty() {
        if format != "json" {
            println!("{}", "No orphaned records found.".green());
        } else {
            println!("[]");
        }
        return Ok(());
    }

    match format {
        "json" => {
            let output: Vec<_> = orphans
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.id(),
                        "title": r.title(),
                        "type": r.record_type().to_string(),
                        "status": r.status().to_string(),
                        "created": r.frontmatter.created.to_string(),
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            println!(
                "{} {} orphaned records (no links):\n",
                "Found".yellow().bold(),
                orphans.len()
            );

            for record in orphans {
                println!(
                    "{} {} [{}]",
                    record.id().cyan(),
                    record.title(),
                    record.status()
                );
                println!("  {} {}", "Created:".dimmed(), record.frontmatter.created);
            }
        }
    }

    Ok(())
}
