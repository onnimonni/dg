use crate::models::Graph;
use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, id: &str, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    // Check record exists
    graph
        .get(id)
        .ok_or_else(|| anyhow!("Record not found: {}", id))?;

    let paths = graph.trace_dependencies(id);

    match format {
        "json" => {
            let output: Vec<_> = paths
                .iter()
                .map(|p| {
                    serde_json::json!({
                        "path": p.nodes,
                        "link_types": p.link_types,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            if paths.is_empty() {
                println!(
                    "{} has no dependencies (via depends_on links)",
                    id.cyan().bold()
                );
                return Ok(());
            }

            println!(
                "{} {} depends on:\n",
                "Why".green().bold(),
                id.cyan().bold()
            );

            for path in &paths {
                print_path(&graph, path);
            }
        }
    }

    Ok(())
}

fn print_path(graph: &Graph, path: &crate::models::DependencyPath) {
    let mut parts = Vec::new();
    for (i, node_id) in path.nodes.iter().enumerate() {
        if let Some(record) = graph.get(node_id) {
            let node_str = format!("{} ({})", node_id.cyan(), record.title());
            parts.push(node_str);

            if i < path.link_types.len() {
                parts.push(
                    format!(" --[{}]--> ", path.link_types[i])
                        .dimmed()
                        .to_string(),
                );
            }
        }
    }
    println!("  {}", parts.join(""));
    println!();
}
