use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, query: &str, include_content: bool) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let results = graph.search(query, include_content);

    if results.is_empty() {
        println!("{}", "No records found.".yellow());
        return Ok(());
    }

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

    Ok(())
}
