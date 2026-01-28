use crate::models::{graph_to_d2, D2Renderer, Graph};
use anyhow::Result;
use colored::Colorize;
use std::collections::HashSet;
use std::path::Path;

pub fn run(
    docs_dir: &str,
    id: Option<&str>,
    depth: usize,
    format: &str,
    render: Option<&str>,
    output: Option<&str>,
) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let subset: Option<HashSet<String>> = id.map(|id| graph.neighbors(id, depth));

    match format {
        "d2" => {
            let d2_source = graph_to_d2(&graph, subset.as_ref());

            match render {
                Some("svg") => {
                    let renderer = D2Renderer::new()?;
                    let svg = renderer.render_svg(&d2_source)?;

                    if let Some(out_path) = output {
                        std::fs::write(out_path, &svg)?;
                        println!("SVG written to {}", out_path);
                    } else {
                        println!("{}", svg);
                    }
                }
                Some(fmt) => {
                    anyhow::bail!("Unknown render format: {}. Supported: svg", fmt);
                }
                None => {
                    println!("{}", d2_source);
                }
            }
        }
        "dot" => {
            println!("{}", graph.to_dot(subset.as_ref()));
        }
        "json" => {
            let nodes: Vec<_> = graph
                .all_records()
                .filter(|r| subset.as_ref().is_none_or(|s| s.contains(r.id())))
                .map(|r| {
                    serde_json::json!({
                        "id": r.id(),
                        "title": r.title(),
                        "type": r.record_type().to_string(),
                    })
                })
                .collect();

            let edges: Vec<_> = graph
                .edges
                .iter()
                .filter(|e| {
                    subset
                        .as_ref()
                        .is_none_or(|s| s.contains(&e.from) && s.contains(&e.to))
                })
                .map(|e| {
                    serde_json::json!({
                        "from": e.from,
                        "to": e.to,
                        "type": e.link_type,
                    })
                })
                .collect();

            let output = serde_json::json!({
                "nodes": nodes,
                "edges": edges,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            // Text format - show as tree/list
            if let (Some(start_id), Some(ref subset)) = (id, &subset) {
                println!(
                    "{} {} (depth={})\n",
                    "Graph from".dimmed(),
                    start_id.cyan().bold(),
                    depth
                );
                print_tree(&graph, start_id, subset, 0);
            } else {
                println!("{}\n", "Full graph:".bold());

                // Group by type
                let mut types: Vec<_> = graph
                    .records
                    .values()
                    .map(|r| r.record_type().to_string())
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();
                types.sort();

                for record_type in types {
                    let records: Vec<_> = graph
                        .records
                        .values()
                        .filter(|r| r.record_type().to_string() == record_type)
                        .collect();

                    println!("{} ({}):", record_type.yellow().bold(), records.len());
                    for record in records {
                        let outgoing = graph.outgoing_edges(record.id());
                        let incoming = graph.incoming_edges(record.id());

                        print!("  {} {}", record.id().cyan(), record.title());
                        if !outgoing.is_empty() || !incoming.is_empty() {
                            print!(
                                " {}",
                                format!("[{} out, {} in]", outgoing.len(), incoming.len()).dimmed()
                            );
                        }
                        println!();
                    }
                    println!();
                }
            }
        }
    }

    Ok(())
}

fn print_tree(graph: &Graph, id: &str, subset: &HashSet<String>, indent: usize) {
    let prefix = "  ".repeat(indent);

    if let Some(record) = graph.get(id) {
        println!("{}{} {}", prefix, record.id().cyan(), record.title());

        let outgoing = graph.outgoing_edges(id);
        for edge in outgoing {
            if subset.contains(&edge.to) {
                println!(
                    "{}  {} ",
                    prefix,
                    format!("--[{}]-->", edge.link_type).dimmed()
                );
                // Avoid infinite recursion for cycles
                if indent < 5 {
                    print_tree(graph, &edge.to, subset, indent + 2);
                } else {
                    println!("{}    {} (depth limit)", prefix, edge.to.cyan());
                }
            }
        }
    }
}
