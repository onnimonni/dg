use crate::models::{Graph, RecordType, Status};
use anyhow::Result;
use chrono::NaiveDate;
use colored::Colorize;
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

pub fn run(
    docs_dir: &str,
    since: Option<&str>,
    type_filter: Option<String>,
    status_filter: Option<String>,
    format: &str,
) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    // Parse the since date (either YYYY-MM-DD or git tag)
    let since_date = match since {
        Some(s) => Some(parse_since(s)?),
        None => None,
    };

    // Parse type filter (comma-separated)
    let type_filters: Vec<RecordType> = type_filter
        .map(|t| {
            t.split(',')
                .filter_map(|s| RecordType::from_str(s.trim()))
                .collect()
        })
        .unwrap_or_default();

    // Parse status filter
    let status_filter = status_filter.and_then(|s| Status::from_str(&s));

    // Collect and filter records
    let mut records: Vec<_> = graph
        .all_records()
        .filter(|r| {
            // Filter by date (created or updated since)
            if let Some(date) = since_date {
                if r.frontmatter.created < date && r.frontmatter.updated < date {
                    return false;
                }
            }
            true
        })
        .filter(|r| {
            // Filter by type
            type_filters.is_empty() || type_filters.contains(r.record_type())
        })
        .filter(|r| {
            // Filter by status
            status_filter.as_ref().is_none_or(|s| r.status() == s)
        })
        .collect();

    // Sort by created date (newest first)
    records.sort_by(|a, b| b.frontmatter.created.cmp(&a.frontmatter.created));

    if records.is_empty() {
        if format == "json" {
            println!("{{\"changes\": []}}");
        } else {
            println!("{}", "No changes found.".yellow());
        }
        return Ok(());
    }

    match format {
        "json" => output_json(&records, since_date),
        _ => output_markdown(&records, since_date),
    }
}

/// Parse since parameter - either a date (YYYY-MM-DD) or git tag
fn parse_since(since: &str) -> Result<NaiveDate> {
    // Try parsing as date first
    if let Ok(date) = NaiveDate::parse_from_str(since, "%Y-%m-%d") {
        return Ok(date);
    }

    // Try as git tag
    let output = Command::new("git")
        .args(["log", "-1", "--format=%ci", since])
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Could not parse '{}' as date (YYYY-MM-DD) or git tag",
            since
        );
    }

    let date_str = String::from_utf8(output.stdout)?;
    let date_str = date_str.trim();

    // Git date format: 2024-01-15 10:30:00 +0000
    let date_part = date_str.split_whitespace().next().unwrap_or(date_str);
    NaiveDate::parse_from_str(date_part, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!("Failed to parse git tag date: {}", e))
}

fn output_markdown(
    records: &[&crate::models::Record],
    since_date: Option<NaiveDate>,
) -> Result<()> {
    println!("# Changelog\n");

    if let Some(date) = since_date {
        println!("Changes since {}\n", date);
    }

    // Group by month (YYYY-MM)
    let mut by_month: BTreeMap<String, Vec<&crate::models::Record>> = BTreeMap::new();
    for record in records {
        let month = record.frontmatter.created.format("%Y-%m").to_string();
        by_month.entry(month).or_default().push(record);
    }

    // Output in reverse chronological order (newest first)
    for (month, month_records) in by_month.into_iter().rev() {
        // Parse month for display
        let date = NaiveDate::parse_from_str(&format!("{}-01", month), "%Y-%m-%d")?;
        println!("## {}\n", date.format("%B %Y"));

        // Group by type within month
        let mut by_type: BTreeMap<String, Vec<&crate::models::Record>> = BTreeMap::new();
        for record in month_records {
            let display_name = record.record_type().display_name().to_string();
            by_type.entry(display_name).or_default().push(record);
        }

        for (type_name, type_records) in by_type {
            // Use plural form for section headers
            let section_name = match type_name.as_str() {
                "Decision" => "Decisions",
                "Strategy" => "Strategies",
                "Policy" => "Policies",
                "Customer" => "Customers",
                "Opportunity" => "Opportunities",
                "Process" => "Processes",
                "Hiring" => "Hiring",
                "Architecture" => "Architecture",
                "Incident" => "Incidents",
                "Runbook" => "Runbooks",
                "Meeting" => "Meetings",
                "Feedback" => "Feedback",
                "Legal" => "Legal",
                _ => &type_name,
            };
            println!("### {}\n", section_name);

            for record in type_records {
                println!(
                    "- **{}**: {} ({})",
                    record.id(),
                    record.title(),
                    record.status()
                );
            }
            println!();
        }
    }

    Ok(())
}

fn output_json(records: &[&crate::models::Record], since_date: Option<NaiveDate>) -> Result<()> {
    // Group by month
    let mut by_month: BTreeMap<String, Vec<serde_json::Value>> = BTreeMap::new();

    for record in records {
        let month = record.frontmatter.created.format("%Y-%m").to_string();
        let entry = serde_json::json!({
            "id": record.id(),
            "title": record.title(),
            "type": record.record_type().to_string(),
            "status": record.status().to_string(),
            "created": record.frontmatter.created.to_string(),
            "updated": record.frontmatter.updated.to_string(),
        });
        by_month.entry(month).or_default().push(entry);
    }

    let output = serde_json::json!({
        "since": since_date.map(|d| d.to_string()),
        "total": records.len(),
        "by_month": by_month,
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_since_date() {
        let date = parse_since("2024-01-15").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
    }

    #[test]
    fn test_parse_since_invalid() {
        // Invalid date format
        assert!(parse_since("not-a-date").is_err());
    }
}
