use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let core = graph.core_records();

    if core.is_empty() {
        println!(
            "{}",
            "No core records found. Mark records with 'core: true' in frontmatter.".yellow()
        );
        return Ok(());
    }

    match format {
        "json" => {
            let output: Vec<_> = core
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
            println!("{} {} core records:\n", "Found".green(), core.len());
            for record in core {
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
