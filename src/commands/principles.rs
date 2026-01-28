use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let foundational = graph.foundational_records();

    if foundational.is_empty() {
        println!(
            "{}",
            "No foundational records found. Mark records with 'foundational: true' in frontmatter."
                .yellow()
        );
        return Ok(());
    }

    match format {
        "json" => {
            let output: Vec<_> = foundational
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.id(),
                        "title": r.title(),
                        "type": r.record_type().to_string(),
                        "status": r.status().to_string(),
                        "tags": r.frontmatter.tags,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            println!(
                "{} {} foundational records:\n",
                "Found".green(),
                foundational.len()
            );
            for record in foundational {
                println!(
                    "{} {} [{}]",
                    record.id().cyan().bold(),
                    record.title(),
                    record.status()
                );
                if !record.frontmatter.tags.is_empty() {
                    println!(
                        "  {}",
                        record
                            .frontmatter
                            .tags
                            .iter()
                            .map(|t| format!("#{}", t).yellow().to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                }
            }
        }
    }

    Ok(())
}
