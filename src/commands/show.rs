use crate::models::{Graph, Status};
use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;

pub fn run(docs_dir: &str, id: &str, show_links: bool, as_json: bool) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let record = graph
        .get(id)
        .ok_or_else(|| anyhow!("Record not found: {}", id))?;

    if as_json {
        let json = serde_json::json!({
            "id": record.id(),
            "title": record.title(),
            "type": record.record_type().to_string(),
            "status": record.status().to_string(),
            "created": record.frontmatter.created.to_string(),
            "updated": record.frontmatter.updated.to_string(),
            "authors": record.frontmatter.authors,
            "tags": record.frontmatter.tags,
            "links": {
                "supersedes": record.frontmatter.links.supersedes,
                "superseded_by": record.frontmatter.links.superseded_by,
                "depends_on": record.frontmatter.links.depends_on,
                "enables": record.frontmatter.links.enables,
                "relates_to": record.frontmatter.links.relates_to,
                "conflicts_with": record.frontmatter.links.conflicts_with,
                "refines": record.frontmatter.links.refines,
                "implements": record.frontmatter.links.implements,
            },
            "path": record.path.to_string_lossy(),
            "content": record.content.trim(),
        });
        println!("{}", serde_json::to_string_pretty(&json)?);
        return Ok(());
    }

    // Header
    let core_marker = if record.frontmatter.foundational {
        format!(" {}", "★ CORE".yellow().bold())
    } else {
        String::new()
    };
    println!("{}{}", record.id().cyan().bold(), core_marker);
    println!("{}", record.title().bold());
    println!("{}", "=".repeat(60));

    // Metadata
    let status_colored = match record.status() {
        Status::Accepted | Status::Active => record.status().to_string().green(),
        Status::Deprecated | Status::Superseded | Status::Cancelled | Status::Open => {
            record.status().to_string().red()
        }
        Status::Draft | Status::Proposed => record.status().to_string().yellow(),
        Status::Resolved => record.status().to_string().blue(),
        _ => record.status().to_string().normal(),
    };
    println!(
        "{}: {}  {}: {}",
        "Type".dimmed(),
        record.record_type(),
        "Status".dimmed(),
        status_colored
    );
    println!(
        "{}: {}  {}: {}",
        "Created".dimmed(),
        record.frontmatter.created,
        "Updated".dimmed(),
        record.frontmatter.updated
    );

    if !record.frontmatter.authors.is_empty() {
        println!(
            "{}: {}",
            "Authors".dimmed(),
            record.frontmatter.authors.join(", ")
        );
    }

    if !record.frontmatter.tags.is_empty() {
        println!(
            "{}: {}",
            "Tags".dimmed(),
            record
                .frontmatter
                .tags
                .iter()
                .map(|t| format!("[{}]", t).green().to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }

    // Links
    let links = record.frontmatter.links.all_links();
    if !links.is_empty() {
        println!("\n{}", "Links:".yellow());
        for (link_type, target) in &links {
            let target_title = graph
                .get(target)
                .map(|r| r.title())
                .unwrap_or("[not found]");
            println!(
                "  {} {} ({})",
                link_type.cyan(),
                target,
                target_title.dimmed()
            );
        }
    }

    // Show linked records if requested
    if show_links {
        let incoming = graph.incoming_edges(id);
        if !incoming.is_empty() {
            println!("\n{}", "Incoming links:".yellow());
            for edge in incoming {
                let from_title = graph
                    .get(&edge.from)
                    .map(|r| r.title())
                    .unwrap_or("[not found]");
                println!(
                    "  {} <- {} ({})",
                    edge.link_type.cyan(),
                    edge.from,
                    from_title.dimmed()
                );
            }
        }
    }

    // Content preview
    let content = record.content.trim();
    if !content.is_empty() {
        println!("\n{}", "-".repeat(60));
        // Show first 20 lines
        let preview: String = content.lines().take(20).collect::<Vec<_>>().join("\n");
        println!("{}", preview);
        let total_lines = content.lines().count();
        if total_lines > 20 {
            println!(
                "\n{}",
                format!("... ({} more lines)", total_lines - 20).dimmed()
            );
        }
    }

    println!("\n{}: {}", "File".dimmed(), record.path.display());

    Ok(())
}
