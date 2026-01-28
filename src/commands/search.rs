use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(
    docs_dir: &str,
    query: &str,
    include_content: bool,
    tag_filter: Option<&str>,
    format: &str,
) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let mut results = graph.search(query, include_content);

    // Filter by tag if specified
    if let Some(tag) = tag_filter {
        let tag_lower = tag.to_lowercase();
        results.retain(|r| {
            r.frontmatter
                .tags
                .iter()
                .any(|t| t.to_lowercase().contains(&tag_lower))
        });
    }

    if results.is_empty() {
        if format != "json" {
            println!("{}", "No records found.".yellow());
        } else {
            println!("[]");
        }
        return Ok(());
    }

    match format {
        "json" => {
            let output: Vec<_> = results
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.id(),
                        "title": r.title(),
                        "type": r.record_type().to_string(),
                        "status": r.status().to_string(),
                        "tags": r.frontmatter.tags,
                        "foundational": r.frontmatter.foundational,
                        "path": r.path.to_string_lossy(),
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        "ids" => {
            for record in results {
                println!("{}", record.id());
            }
        }
        _ => {
            println!(
                "{} {} results for '{}':\n",
                "Found".green(),
                results.len(),
                query.cyan()
            );

            for record in results {
                println!(
                    "{} {} [{}]",
                    record.id().cyan().bold(),
                    record.title(),
                    record.status()
                );

                // Show matching tags
                let matching_tags: Vec<_> = record
                    .frontmatter
                    .tags
                    .iter()
                    .filter(|t| t.to_lowercase().contains(&query.to_lowercase()))
                    .collect();
                if !matching_tags.is_empty() {
                    println!(
                        "  Tags: {}",
                        matching_tags
                            .iter()
                            .map(|t| format!("#{}", t).yellow().to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                }

                // Show content snippet if searching content
                if include_content {
                    let query_lower = query.to_lowercase();
                    let content_lower = record.content.to_lowercase();
                    if let Some(pos) = content_lower.find(&query_lower) {
                        let start = pos.saturating_sub(40);
                        let end = (pos + query.len() + 40).min(record.content.len());
                        let snippet = &record.content[start..end];
                        let snippet = snippet
                            .replace('\n', " ")
                            .chars()
                            .take(100)
                            .collect::<String>();
                        println!("  ...{}...", snippet.dimmed());
                    }
                }

                println!();
            }
        }
    }

    Ok(())
}
