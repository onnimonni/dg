use crate::models::{Graph, RecordType};
use anyhow::{anyhow, Result};
use chrono::Local;
use colored::Colorize;
use minijinja::{Environment, UndefinedBehavior};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, IsTerminal, Write};
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

/// Get the current git user name, falling back to system username
fn get_author() -> String {
    // Try git config user.name first
    if let Ok(output) = std::process::Command::new("git")
        .args(["config", "user.name"])
        .output()
    {
        if output.status.success() {
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !name.is_empty() {
                return name;
            }
        }
    }

    // Fall back to system username
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}

/// Parse --var key=value arguments into a HashMap
fn parse_vars(vars: &[String]) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    for var in vars {
        if let Some((key, value)) = var.split_once('=') {
            map.insert(key.to_string(), value.to_string());
        } else {
            return Err(anyhow!(
                "Invalid variable format '{}'. Use --var key=value",
                var
            ));
        }
    }
    Ok(map)
}

/// Extract variable names from a template using regex
/// Finds {{ varname }} patterns, excluding filters like {{ var | filter }}
fn extract_template_variables(template: &str) -> HashSet<String> {
    let re = Regex::new(r"\{\{\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*(?:\|[^}]*)?\}\}").unwrap();
    re.captures_iter(template)
        .map(|cap| cap[1].to_string())
        .collect()
}

/// Prompt user for a variable value interactively
fn prompt_for_variable(name: &str) -> Result<String> {
    print!("{} Enter value for '{}': ", "?".cyan().bold(), name);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn run(
    docs_dir: &str,
    record_type: &str,
    title: &str,
    draft: bool,
    vars: Vec<String>,
) -> Result<()> {
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

    // Parse user-provided variables
    let mut user_vars = parse_vars(&vars)?;

    // Prepare built-in variables
    let today = Local::now().format("%Y-%m-%d").to_string();
    let author = get_author();

    // Escape title for YAML - wrap in quotes if it contains special chars
    let yaml_title = if title.contains(':') || title.contains('#') || title.contains('"') {
        format!("\"{}\"", title.replace('"', "\\\""))
    } else {
        title.to_string()
    };

    // Extract number for non-draft IDs
    let number_str = if draft {
        "NEW".to_string()
    } else {
        new_id.split('-').nth(1).unwrap_or("001").to_string()
    };

    // Built-in variables (these take precedence for backwards compatibility)
    let mut builtin_vars: HashMap<String, String> = HashMap::new();
    builtin_vars.insert("id".to_string(), new_id.clone());
    builtin_vars.insert("title".to_string(), yaml_title.clone());
    builtin_vars.insert("date".to_string(), today.clone());
    builtin_vars.insert("author".to_string(), author);

    // Legacy placeholders for backwards compatibility
    builtin_vars.insert("NUMBER".to_string(), number_str.clone());
    builtin_vars.insert("TITLE".to_string(), yaml_title.clone());
    builtin_vars.insert("DATE".to_string(), today.clone());
    builtin_vars.insert("CLIENT_NAME".to_string(), yaml_title.clone());
    builtin_vars.insert("ROLE_TITLE".to_string(), yaml_title.clone());
    builtin_vars.insert("NAME".to_string(), "Option".to_string());

    // Extract all variables from template
    let template_vars = extract_template_variables(&template);

    // Find missing variables (not in builtin or user-provided)
    let all_provided: HashSet<String> = builtin_vars
        .keys()
        .chain(user_vars.keys())
        .cloned()
        .collect();
    let missing: Vec<String> = template_vars.difference(&all_provided).cloned().collect();

    // Handle missing variables
    if !missing.is_empty() {
        if io::stdin().is_terminal() && io::stdout().is_terminal() {
            // Interactive mode: prompt for missing variables
            for var_name in &missing {
                let value = prompt_for_variable(var_name)?;
                user_vars.insert(var_name.clone(), value);
            }
        } else {
            // Non-interactive mode: error out
            return Err(anyhow!(
                "Missing required template variables: {}. Use --var to provide them.",
                missing.join(", ")
            ));
        }
    }

    // First, handle the special id: PREFIX-{{NUMBER}} pattern that needs exact replacement
    let content = template.replace(
        &format!("id: {}-{{{{NUMBER}}}}", rt.prefix()),
        &format!("id: {}", new_id),
    );

    // Create minijinja environment and render template
    let mut env = Environment::new();
    env.set_undefined_behavior(UndefinedBehavior::Strict);

    // Build context with all variables
    let mut context: HashMap<String, String> = builtin_vars;
    context.extend(user_vars);

    // Add template and render
    env.add_template("record", &content)?;
    let tmpl = env.get_template("record")?;
    let rendered = tmpl.render(&context)?;

    fs::write(&file_path, rendered)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vars() {
        let vars = vec!["team=platform".to_string(), "priority=high".to_string()];
        let result = parse_vars(&vars).unwrap();
        assert_eq!(result.get("team"), Some(&"platform".to_string()));
        assert_eq!(result.get("priority"), Some(&"high".to_string()));
    }

    #[test]
    fn test_parse_vars_with_equals_in_value() {
        let vars = vec!["equation=a=b+c".to_string()];
        let result = parse_vars(&vars).unwrap();
        assert_eq!(result.get("equation"), Some(&"a=b+c".to_string()));
    }

    #[test]
    fn test_parse_vars_invalid() {
        let vars = vec!["invalid".to_string()];
        assert!(parse_vars(&vars).is_err());
    }

    #[test]
    fn test_extract_template_variables() {
        let template = "Hello {{ name }}, today is {{ date }}. Your team is {{ team | upper }}.";
        let vars = extract_template_variables(template);
        assert!(vars.contains("name"));
        assert!(vars.contains("date"));
        assert!(vars.contains("team"));
    }

    #[test]
    fn test_extract_template_variables_no_duplicates() {
        let template = "{{ name }} {{ name }} {{ name }}";
        let vars = extract_template_variables(template);
        assert_eq!(vars.len(), 1);
        assert!(vars.contains("name"));
    }

    #[test]
    fn test_is_draft_id() {
        assert!(is_draft_id("DEC-NEW-20240115120000"));
        assert!(!is_draft_id("DEC-001"));
    }
}
