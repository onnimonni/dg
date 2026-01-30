use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, query: &str, depth: usize, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let ctx = graph.context(query, depth);

    if ctx.records.is_empty() {
        println!("{}", "No matching records found.".yellow());
        return Ok(());
    }

    match format {
        "json" => {
            let output = serde_json::json!({
                "query": query,
                "depth": depth,
                "records": ctx.records.iter().map(|r| {
                    serde_json::json!({
                        "id": r.id(),
                        "title": r.title(),
                        "type": r.record_type().to_string(),
                        "status": r.status().to_string(),
                        "tags": r.frontmatter.tags,
                        "core": r.frontmatter.core,
                        "summary": extract_summary(&r.content),
                    })
                }).collect::<Vec<_>>(),
                "edges": ctx.edges.iter().map(|e| {
                    serde_json::json!({
                        "from": e.from,
                        "to": e.to,
                        "type": e.link_type,
                    })
                }).collect::<Vec<_>>(),
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            println!("{} for '{}':\n", "Context".green().bold(), query.cyan());

            // Group by core
            let (core, regular): (Vec<_>, Vec<_>) =
                ctx.records.into_iter().partition(|r| r.frontmatter.core);

            if !core.is_empty() {
                println!("{}", "Foundational:".yellow().bold());
                for record in core {
                    println!(
                        "  {} {} [{}]",
                        record.id().cyan().bold(),
                        record.title(),
                        record.status()
                    );
                }
                println!();
            }

            println!("{}", "Related:".bold());
            for record in regular {
                println!(
                    "  {} {} [{}]",
                    record.id().cyan(),
                    record.title(),
                    record.status()
                );
            }

            if !ctx.edges.is_empty() {
                println!("\n{}", "Connections:".bold());
                for edge in ctx.edges {
                    println!(
                        "  {} --[{}]--> {}",
                        edge.from.cyan(),
                        edge.link_type.dimmed(),
                        edge.to.cyan()
                    );
                }
            }
        }
    }

    Ok(())
}

fn extract_summary(content: &str) -> String {
    // Get first non-empty line that isn't a heading
    content
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .take(1)
        .collect::<Vec<_>>()
        .join("")
        .chars()
        .take(200)
        .collect()
}
