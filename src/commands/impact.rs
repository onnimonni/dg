use crate::models::Graph;
use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, id: &str, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    // Check record exists
    let record = graph
        .get(id)
        .ok_or_else(|| anyhow!("Record not found: {}", id))?;

    let paths = graph.trace_dependents(id);

    match format {
        "json" => {
            let output = serde_json::json!({
                "id": id,
                "title": record.title(),
                "foundational": record.frontmatter.foundational,
                "dependents": paths.iter().map(|p| {
                    serde_json::json!({
                        "path": p.nodes,
                        "link_types": p.link_types,
                    })
                }).collect::<Vec<_>>(),
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            println!(
                "{} of changing {}:\n",
                "Impact".red().bold(),
                id.cyan().bold()
            );

            if record.frontmatter.foundational {
                println!(
                    "  {} This is a {} record!\n",
                    "⚠".yellow(),
                    "foundational".yellow().bold()
                );
            }

            if paths.is_empty() {
                println!("  No records depend on this (via depends_on links)");
                return Ok(());
            }

            println!(
                "  {} records would be affected:\n",
                paths.len().to_string().red()
            );

            for path in &paths {
                print_path(&graph, path);
            }
        }
    }

    Ok(())
}

fn print_path(graph: &Graph, path: &crate::models::DependencyPath) {
    // Skip the first node (it's the source)
    for (i, node_id) in path.nodes.iter().skip(1).enumerate() {
        let indent = "  ".repeat(i + 1);
        if let Some(record) = graph.get(node_id) {
            println!("{}← {} {}", indent, node_id.cyan(), record.title().dimmed());
        }
    }
}
