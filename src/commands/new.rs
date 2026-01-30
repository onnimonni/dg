use crate::models::{Graph, RecordType};
use anyhow::{anyhow, Result};
use chrono::Local;
use colored::Colorize;
use std::fs;
use std::path::Path;

/// Generate a draft ID with timestamp (for multi-player mode)
pub fn draft_id(record_type: &RecordType) -> String {
    let prefix = record_type.prefix();
    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    format!("{}-NEW-{}", prefix, timestamp)
}

/// Check if an ID is a draft ID
pub fn is_draft_id(id: &str) -> bool {
    id.contains("-NEW-")
}

pub fn run(docs_dir: &str, record_type: &str, title: &str, draft: bool) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let decisions_path = docs_path.join("decisions");
    let templates_path = docs_path.join(".templates");

    if !decisions_path.exists() {
        return Err(anyhow!(
            "Decision graph not initialized. Run 'dg init' first."
        ));
    }

    let rt = RecordType::from_str(record_type)
        .ok_or_else(|| anyhow!("Unknown record type: {}", record_type))?;

    // Load graph to get next ID
    let graph = Graph::load(docs_path)?;
    let new_id = if draft {
        draft_id(&rt)
    } else {
        graph.next_id(&rt)
    };

    // Load template
    let template_path = templates_path.join(format!("{}.md", rt.template_name()));
    let template = if template_path.exists() {
        fs::read_to_string(&template_path)?
    } else {
        default_template(&rt)
    };

    // Generate filename
    let slug = title
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();
    let filename = format!("{}-{}.md", new_id, slug);
    let file_path = decisions_path.join(&filename);

    // Substitute placeholders
    let today = Local::now().format("%Y-%m-%d").to_string();
    // Escape title for YAML - wrap in quotes if it contains special chars
    let yaml_title = if title.contains(':') || title.contains('#') || title.contains('"') {
        format!("\"{}\"", title.replace('"', "\\\""))
    } else {
        title.to_string()
    };
    // Fix the ID in frontmatter FIRST (before NUMBER replacement)
    let content = template.replace(
        &format!("id: {}-{{{{NUMBER}}}}", rt.prefix()),
        &format!("id: {}", new_id),
    );

    // Extract number for non-draft IDs (for other template placeholders)
    let number_str = if draft {
        "NEW".to_string()
    } else {
        new_id.split('-').nth(1).unwrap_or("001").to_string()
    };

    let content = content
        .replace("{{NUMBER}}", &number_str)
        .replace("{{TITLE}}", &yaml_title)
        .replace("{{DATE}}", &today)
        .replace("{{CLIENT_NAME}}", &yaml_title)
        .replace("{{ROLE_TITLE}}", &yaml_title)
        .replace("{{NAME}}", "Option");

    fs::write(&file_path, content)?;

    println!("{} {}", "Created".green().bold(), file_path.display());
    if draft {
        println!(
            "Draft ID: {} (use 'dg finalize' before merging)",
            new_id.yellow()
        );
    } else {
        println!("ID: {}", new_id.cyan());
    }

    // Reload and update index
    let graph = Graph::load(docs_path)?;
    let _ = graph.save_index();

    Ok(())
}

fn default_template(rt: &RecordType) -> String {
    let prefix = rt.prefix();
    format!(
        r#"---
type: {}
id: {}-{{{{NUMBER}}}}
title: {{{{TITLE}}}}
status: proposed
created: {{{{DATE}}}}
updated: {{{{DATE}}}}
authors: []
tags: []
links:
  supersedes: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
---

# {{{{TITLE}}}}

## Context


## Decision


## Consequences

"#,
        rt.template_name(),
        prefix
    )
}
