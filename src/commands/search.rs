use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use regex::Regex;
use std::path::Path;

/// Parsed search query with filters
#[derive(Debug, Default)]
struct Query {
    text: Vec<String>,
    type_filter: Option<String>,
    status_filter: Option<String>,
    tag_filter: Option<String>,
    author_filter: Option<String>,
    title_filter: Option<String>,
    id_filter: Option<String>,
    core: Option<bool>,
}

impl Query {
    fn parse(input: &str) -> Self {
        let mut query = Query::default();
        let filter_re = Regex::new(r"(\w+):(\S+)").unwrap();

        let mut remaining = input.to_string();

        // Extract filters
        for cap in filter_re.captures_iter(input) {
            let key = cap.get(1).unwrap().as_str().to_lowercase();
            let value = cap.get(2).unwrap().as_str().to_string();

            match key.as_str() {
                "type" | "t" => query.type_filter = Some(value.to_lowercase()),
                "status" | "s" => query.status_filter = Some(value.to_lowercase()),
                "tag" => query.tag_filter = Some(value.to_lowercase()),
                "author" | "a" => query.author_filter = Some(value.to_lowercase()),
                "title" => query.title_filter = Some(value.to_lowercase()),
                "id" => query.id_filter = Some(value.to_lowercase()),
                "core" | "f" => {
                    query.core = Some(value == "true" || value == "yes" || value == "1")
                }
                _ => {}
            }

            // Remove matched filter from remaining text
            remaining = remaining.replace(&cap[0], "");
        }

        // Remaining text becomes free-text search
        let text: Vec<String> = remaining
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect();

        query.text = text;
        query
    }

    #[allow(clippy::overly_complex_bool_expr)]
    fn is_empty(&self) -> bool {
        self.text.is_empty()
            && self.type_filter.is_none()
            && self.status_filter.is_none()
            && self.tag_filter.is_none()
            && self.author_filter.is_none()
            && self.title_filter.is_none()
            && self.id_filter.is_none()
            && self.core.is_none()
    }
}

pub fn run(
    docs_dir: &str,
    query_str: &str,
    include_content: bool,
    tag_filter: Option<&str>,
    format: &str,
) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    let mut query = Query::parse(query_str);

    // CLI tag filter overrides query tag filter
    if let Some(tag) = tag_filter {
        query.tag_filter = Some(tag.to_lowercase());
    }

    // If query is empty and no filters, show help
    if query.is_empty() {
        println!("{}", "Search query syntax:".bold());
        println!("  dg search <text>              Free text search");
        println!("  dg search type:adr            Filter by type");
        println!("  dg search status:accepted     Filter by status");
        println!("  dg search tag:auth            Filter by tag");
        println!("  dg search author:john         Filter by author");
        println!("  dg search title:compress      Filter by title");
        println!("  dg search id:adr-001          Filter by ID");
        println!("  dg search core:true           Filter core records");
        println!();
        println!("{}", "Examples:".bold());
        println!("  dg search auth type:adr");
        println!("  dg search status:accepted tag:security");
        println!("  dg search title:pivot core:true");
        return Ok(());
    }

    let results: Vec<_> = graph
        .all_records()
        .filter(|r| {
            // Type filter
            if let Some(ref t) = query.type_filter {
                let record_type = r.record_type().to_string().to_lowercase();
                let prefix = r.record_type().prefix().to_lowercase();
                if record_type != *t && prefix != *t {
                    return false;
                }
            }

            // Status filter
            if let Some(ref s) = query.status_filter {
                if r.status().to_string().to_lowercase() != *s {
                    return false;
                }
            }

            // Tag filter
            if let Some(ref tag) = query.tag_filter {
                if !r
                    .frontmatter
                    .tags
                    .iter()
                    .any(|t| t.to_lowercase().contains(tag))
                {
                    return false;
                }
            }

            // Author filter
            if let Some(ref author) = query.author_filter {
                if !r
                    .frontmatter
                    .authors
                    .iter()
                    .any(|a| a.to_lowercase().contains(author))
                {
                    return false;
                }
            }

            // Core filter
            if let Some(f) = query.core {
                if r.frontmatter.core != f {
                    return false;
                }
            }

            // Title filter
            if let Some(ref title) = query.title_filter {
                if !r.title().to_lowercase().contains(title) {
                    return false;
                }
            }

            // ID filter
            if let Some(ref id) = query.id_filter {
                if !r.id().to_lowercase().contains(id) {
                    return false;
                }
            }

            // Free text search
            if !query.text.is_empty() {
                let title_lower = r.title().to_lowercase();
                let id_lower = r.id().to_lowercase();
                let content_lower = if include_content {
                    r.content.to_lowercase()
                } else {
                    String::new()
                };

                for term in &query.text {
                    let matches = title_lower.contains(term)
                        || id_lower.contains(term)
                        || r.frontmatter
                            .tags
                            .iter()
                            .any(|t| t.to_lowercase().contains(term))
                        || (include_content && content_lower.contains(term));

                    if !matches {
                        return false;
                    }
                }
            }

            true
        })
        .collect();

    if results.is_empty() {
        if format != "json" {
            println!("{}", "No records found.".yellow());
        } else {
            println!("[]");
        }
        return Ok(());
    }

    match format {
        "json" => {
            let output: Vec<_> = results
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.id(),
                        "title": r.title(),
                        "type": r.record_type().to_string(),
                        "status": r.status().to_string(),
                        "tags": r.frontmatter.tags,
                        "core": r.frontmatter.core,
                        "path": r.path.to_string_lossy(),
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        "ids" => {
            for record in results {
                println!("{}", record.id());
            }
        }
        _ => {
            println!(
                "{} {} results for '{}':\n",
                "Found".green(),
                results.len(),
                query_str.cyan()
            );

            for record in results {
                let core_badge = if record.frontmatter.core {
                    " [CORE]".yellow().to_string()
                } else {
                    String::new()
                };

                println!(
                    "{} {} [{}]{}",
                    record.id().cyan().bold(),
                    record.title(),
                    record.status(),
                    core_badge
                );

                // Show tags
                if !record.frontmatter.tags.is_empty() {
                    println!(
                        "  {}",
                        record
                            .frontmatter
                            .tags
                            .iter()
                            .map(|t| format!("#{}", t).dimmed().to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                }

                // Show content snippet if searching content
                if include_content && !query.text.is_empty() {
                    let content_lower = record.content.to_lowercase();
                    for term in &query.text {
                        if let Some(pos) = content_lower.find(term) {
                            let start = pos.saturating_sub(40);
                            let end = (pos + term.len() + 40).min(record.content.len());
                            let snippet = &record.content[start..end];
                            let snippet = snippet
                                .replace('\n', " ")
                                .chars()
                                .take(100)
                                .collect::<String>();
                            println!("  ...{}...", snippet.dimmed());
                            break;
                        }
                    }
                }

                println!();
            }
        }
    }

    Ok(())
}
