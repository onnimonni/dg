use crate::commands::list::warn_open_incidents;
use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let stats = graph.stats();

    println!("{}", "Decision Graph Statistics".bold());
    println!("{}", "=".repeat(40));

    println!("\n{}: {}", "Total records".cyan(), stats.total_records);
    println!("{}: {}", "Total links".cyan(), stats.total_edges);

    println!("\n{}", "By type:".yellow());
    let mut types: Vec<_> = stats.by_type.iter().collect();
    types.sort_by(|a, b| b.1.cmp(a.1));
    for (record_type, count) in types {
        println!("  {:<12} {}", record_type, count);
    }

    println!("\n{}", "By status:".yellow());
    let mut statuses: Vec<_> = stats.by_status.iter().collect();
    statuses.sort_by(|a, b| b.1.cmp(a.1));
    for (status, count) in statuses {
        let colored_status = match status.as_str() {
            "accepted" | "active" => status.green(),
            "deprecated" | "superseded" | "cancelled" | "open" => status.red(),
            "draft" | "proposed" => status.yellow(),
            "resolved" => status.blue(),
            _ => status.normal(),
        };
        println!("  {:<12} {}", colored_status, count);
    }

    // Calculate some graph metrics
    if stats.total_records > 0 {
        let avg_links = stats.total_edges as f64 / stats.total_records as f64;
        println!("\n{}: {:.1}", "Avg links per record".dimmed(), avg_links);
    }

    // Find most connected records
    let mut connectivity: Vec<_> = graph
        .records
        .keys()
        .map(|id| {
            let out = graph.outgoing_edges(id).len();
            let inc = graph.incoming_edges(id).len();
            (id, out + inc)
        })
        .collect();
    connectivity.sort_by(|a, b| b.1.cmp(&a.1));

    if !connectivity.is_empty() {
        println!("\n{}", "Most connected:".yellow());
        for (id, count) in connectivity.iter().take(5) {
            if *count > 0 {
                let title = graph.get(id).map(|r| r.title()).unwrap_or("");
                println!("  {} {} ({})", id.cyan(), title, count);
            }
        }
    }

    // Warn about open incidents
    warn_open_incidents(&graph);

    Ok(())
}
