use crate::models::{Graph, RecordType, Status};
use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub fn run(
    docs_dir: &str,
    type_filter: Option<String>,
    status_filter: Option<String>,
    tag_filter: Option<String>,
    format: &str,
) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    // Check for open incidents and warn
    warn_open_incidents(&graph);

    let type_filter = type_filter.and_then(|t| RecordType::from_str(&t));
    let status_filter = status_filter.and_then(|s| Status::from_str(&s));

    let mut records: Vec<_> = graph
        .all_records()
        .filter(|r| type_filter.as_ref().is_none_or(|t| r.record_type() == t))
        .filter(|r| status_filter.as_ref().is_none_or(|s| r.status() == s))
        .filter(|r| {
            tag_filter.as_ref().is_none_or(|tag| {
                r.frontmatter
                    .tags
                    .iter()
                    .any(|t| t.to_lowercase().contains(&tag.to_lowercase()))
            })
        })
        .collect();

    // Sort by ID
    records.sort_by(|a, b| a.id().cmp(b.id()));

    match format {
        "json" => {
            let json_records: Vec<_> = records
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.id(),
                        "title": r.title(),
                        "type": r.record_type().to_string(),
                        "status": r.status().to_string(),
                        "tags": r.frontmatter.tags,
                        "created": r.frontmatter.created.to_string(),
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json_records)?);
        }
        "ids" => {
            for record in &records {
                println!("{}", record.id());
            }
        }
        _ => {
            // Table format
            if records.is_empty() {
                println!("{}", "No records found.".yellow());
                return Ok(());
            }

            println!(
                "{:<12} {:<12} {:<10} {:<12} {}",
                "ID".bold(),
                "CREATED".bold(),
                "TYPE".bold(),
                "STATUS".bold(),
                "TITLE".bold()
            );
            println!("{}", "-".repeat(85));

            for record in &records {
                let status_colored = match record.status() {
                    Status::Accepted | Status::Active => record.status().to_string().green(),
                    Status::Deprecated | Status::Superseded | Status::Cancelled | Status::Open => {
                        record.status().to_string().red()
                    }
                    Status::Draft | Status::Proposed => record.status().to_string().yellow(),
                    Status::Resolved => record.status().to_string().blue(),
                    _ => record.status().to_string().normal(),
                };

                let core_marker = if record.frontmatter.foundational {
                    " ★".yellow().to_string()
                } else {
                    String::new()
                };
                print!(
                    "{:<12} {:<12} {:<10} {:<12} {}",
                    record.id().cyan(),
                    record.frontmatter.created.to_string().dimmed(),
                    record.record_type().to_string(),
                    status_colored,
                    truncate(record.title(), 35)
                );
                println!("{}", core_marker);
            }

            println!("\n{} records", records.len());
        }
    }

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Print warning about open incidents to stderr
pub fn warn_open_incidents(graph: &Graph) {
    let open_incidents: Vec<_> = graph
        .all_records()
        .filter(|r| r.record_type() == &RecordType::Incident && r.status() == &Status::Open)
        .collect();

    if !open_incidents.is_empty() {
        let ids: Vec<_> = open_incidents.iter().map(|r| r.id()).collect();
        eprintln!(
            "\n{} {} open incident{}: {}",
            "⚠".yellow(),
            open_incidents.len(),
            if open_incidents.len() == 1 { "" } else { "s" },
            ids.join(", ").red()
        );
    }
}
