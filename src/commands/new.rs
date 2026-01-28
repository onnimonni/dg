use crate::models::{Graph, RecordType};
use anyhow::{anyhow, Result};
use chrono::Local;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn run(docs_dir: &str, record_type: &str, title: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let decisions_path = docs_path.join(".decisions");
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
    let new_id = graph.next_id(&rt);

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
    let content = template
        .replace("{{NUMBER}}", &new_id.split('-').nth(1).unwrap_or("001"))
        .replace("{{TITLE}}", &yaml_title)
        .replace("{{DATE}}", &today)
        .replace("{{CLIENT_NAME}}", &yaml_title)
        .replace("{{ROLE_TITLE}}", &yaml_title)
        .replace("{{NAME}}", "Option");

    // Fix the ID in frontmatter
    let content = content.replace(
        &format!("id: {}-{{{{NUMBER}}}}", rt.prefix()),
        &format!("id: {}", new_id),
    );

    fs::write(&file_path, content)?;

    println!("{} {}", "Created".green().bold(), file_path.display());
    println!("ID: {}", new_id.cyan());

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
