use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, limit: usize, sort_by: &str, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let mut records: Vec<_> = graph.all_records().collect();

    // Sort by date
    match sort_by {
        "created" => records.sort_by(|a, b| b.frontmatter.created.cmp(&a.frontmatter.created)),
        _ => records.sort_by(|a, b| b.frontmatter.updated.cmp(&a.frontmatter.updated)),
    }

    // Limit
    records.truncate(limit);

    if records.is_empty() {
        if format != "json" {
            println!("{}", "No records found.".yellow());
        } else {
            println!("[]");
        }
        return Ok(());
    }

    match format {
        "json" => {
            let output: Vec<_> = records
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.id(),
                        "title": r.title(),
                        "type": r.record_type().to_string(),
                        "status": r.status().to_string(),
                        "created": r.frontmatter.created.to_string(),
                        "updated": r.frontmatter.updated.to_string(),
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            println!(
                "{} (by {}, showing {}):\n",
                "Timeline".cyan().bold(),
                sort_by,
                limit
            );

            let mut current_month = String::new();

            for record in records {
                let date = if sort_by == "created" {
                    record.frontmatter.created
                } else {
                    record.frontmatter.updated
                };

                // Group by month
                let month = date.format("%Y-%m").to_string();
                if month != current_month {
                    if !current_month.is_empty() {
                        println!();
                    }
                    println!("{}", date.format("%B %Y").to_string().bold());
                    current_month = month;
                }

                let type_color = match record.record_type().prefix() {
                    "DEC" => "green",
                    "ADR" => "blue",
                    "INC" => "red",
                    "STR" => "magenta",
                    _ => "white",
                };

                println!(
                    "  {} {} {} [{}]",
                    date.format("%d").to_string().dimmed(),
                    record.id().color(type_color),
                    record.title(),
                    record.status()
                );
            }
        }
    }

    Ok(())
}
