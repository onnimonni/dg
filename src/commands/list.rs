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

    let type_filter = type_filter.and_then(|t| RecordType::from_str(&t));
    let status_filter = status_filter.and_then(|s| Status::from_str(&s));

    let mut records: Vec<_> = graph
        .all_records()
        .filter(|r| {
            type_filter
                .as_ref()
                .map_or(true, |t| r.record_type() == t)
        })
        .filter(|r| {
            status_filter
                .as_ref()
                .map_or(true, |s| r.status() == s)
        })
        .filter(|r| {
            tag_filter.as_ref().map_or(true, |tag| {
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
                .map(|r| serde_json::json!({
                    "id": r.id(),
                    "title": r.title(),
                    "type": r.record_type().to_string(),
                    "status": r.status().to_string(),
                    "tags": r.frontmatter.tags,
                    "created": r.frontmatter.created.to_string(),
                }))
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
                "{:<12} {:<10} {:<12} {}",
                "ID".bold(),
                "TYPE".bold(),
                "STATUS".bold(),
                "TITLE".bold()
            );
            println!("{}", "-".repeat(70));

            for record in &records {
                let status_colored = match record.status() {
                    Status::Accepted | Status::Active => record.status().to_string().green(),
                    Status::Deprecated | Status::Superseded | Status::Cancelled => {
                        record.status().to_string().red()
                    }
                    Status::Draft => record.status().to_string().yellow(),
                    _ => record.status().to_string().normal(),
                };

                println!(
                    "{:<12} {:<10} {:<12} {}",
                    record.id().cyan(),
                    record.record_type().to_string(),
                    status_colored,
                    truncate(record.title(), 40)
                );
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
