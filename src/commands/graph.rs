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
                print_graph_summary(&graph);
            }
        }
    }

    Ok(())
}

fn print_graph_summary(graph: &Graph) {
    let total_records = graph.records.len();
    let total_edges = graph.edges.len();

    // Header
    println!("{}", "═".repeat(60).dimmed());
    println!(
        "{}  {} records, {} links",
        "DECISION GRAPH".bold(),
        total_records.to_string().cyan(),
        total_edges.to_string().cyan()
    );
    println!("{}\n", "═".repeat(60).dimmed());

    // Core/Foundational records
    let core_records: Vec<_> = graph.core_records();
    if !core_records.is_empty() {
        println!(
            "{} ({})",
            "★ CORE RECORDS".yellow().bold(),
            core_records.len()
        );
        for record in &core_records {
            println!(
                "  {} {} {}",
                "★".yellow(),
                record.id().cyan(),
                record.title()
            );
        }
        println!();
    }

    // Most connected records (top 5)
    let mut connectivity: Vec<_> = graph
        .records
        .values()
        .map(|r| {
            let out = graph.outgoing_edges(r.id()).len();
            let inc = graph.incoming_edges(r.id()).len();
            (r, out + inc, out, inc)
        })
        .collect();
    connectivity.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{}", "TOP CONNECTED".bold());
    for (record, total, out, inc) in connectivity.iter().take(5) {
        let bar_len = (*total as f32 / connectivity[0].1 as f32 * 20.0) as usize;
        let bar = "█".repeat(bar_len);
        println!(
            "  {} {:12} {} {} ({} → {} ←)",
            record.id().cyan(),
            record.title().chars().take(30).collect::<String>(),
            bar.green(),
            total,
            out,
            inc
        );
    }
    println!();

    // Orphaned records (no links)
    let orphans: Vec<_> = graph
        .records
        .values()
        .filter(|r| {
            graph.outgoing_edges(r.id()).is_empty() && graph.incoming_edges(r.id()).is_empty()
        })
        .collect();
    if !orphans.is_empty() {
        println!("{} ({})", "⚠ ORPHANED RECORDS".red().bold(), orphans.len());
        for record in &orphans {
            println!("  {} {}", record.id().red(), record.title().dimmed());
        }
        println!();
    }

    // Records by type summary
    println!("{}", "BY TYPE".bold());
    let mut type_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for record in graph.records.values() {
        *type_counts
            .entry(record.record_type().to_string())
            .or_default() += 1;
    }
    let mut type_vec: Vec<_> = type_counts.into_iter().collect();
    type_vec.sort_by(|a, b| b.1.cmp(&a.1));

    for (record_type, count) in type_vec {
        let bar_len = (count as f32 / total_records as f32 * 30.0) as usize;
        let bar = "▓".repeat(bar_len);
        println!("  {:4} {:3} {}", record_type.yellow(), count, bar.dimmed());
    }
    println!();

    // Link types breakdown
    println!("{}", "LINK TYPES".bold());
    let mut link_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for edge in &graph.edges {
        *link_counts.entry(edge.link_type.clone()).or_default() += 1;
    }
    let mut link_vec: Vec<_> = link_counts.into_iter().collect();
    link_vec.sort_by(|a, b| b.1.cmp(&a.1));

    for (link_type, count) in link_vec {
        println!("  {:15} {}", link_type.dimmed(), count);
    }
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
