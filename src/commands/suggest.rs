use crate::models::Graph;
use anyhow::Result;
use colored::Colorize;
use regex::Regex;
use std::path::Path;
use std::process::Command;

const DECISION_KEYWORDS: &[&str] = &[
    "decide",
    "decided",
    "decision",
    "chose",
    "choose",
    "choosing",
    "adopt",
    "adopted",
    "adopting",
    "migrate",
    "migrated",
    "migration",
    "switch",
    "switched",
    "switching",
    "replace",
    "replaced",
    "replacing",
    "deprecate",
    "deprecated",
    "deprecating",
    "remove",
    "removed",
    "removing",
    "add",
    "added",
    "adding",
    "implement",
    "implemented",
    "implementing",
    "refactor",
    "refactored",
    "refactoring",
    "rewrite",
    "rewrote",
    "rewriting",
];

const ARCHITECTURE_KEYWORDS: &[&str] = &[
    "architecture",
    "design",
    "pattern",
    "structure",
    "framework",
    "library",
    "dependency",
    "api",
    "interface",
    "protocol",
    "database",
    "storage",
    "cache",
    "queue",
    "event",
];

const INCIDENT_KEYWORDS: &[&str] = &[
    "fix", "fixed", "hotfix", "urgent", "critical", "bug", "crash", "error", "failure", "outage",
    "incident", "revert", "reverted", "rollback",
];

pub fn run(docs_dir: &str, since: Option<&str>, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    // Get git commits
    let since_arg = since.unwrap_or("1 week ago");
    let output = Command::new("git")
        .args(["log", "--oneline", "--since", since_arg])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git log failed: {}", stderr);
    }

    let commits = String::from_utf8(output.stdout)?;
    let suggestions = analyze_commits(&commits, &graph);

    if suggestions.is_empty() {
        if format != "json" {
            println!("{}", "No missing decisions detected.".green());
        } else {
            println!("[]");
        }
        return Ok(());
    }

    match format {
        "json" => {
            let output: Vec<_> = suggestions
                .iter()
                .map(|s| {
                    serde_json::json!({
                        "commit": s.commit,
                        "message": s.message,
                        "suggested_type": s.suggested_type,
                        "reason": s.reason,
                        "related": s.related,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            println!(
                "{} {} potential missing records:\n",
                "Found".yellow().bold(),
                suggestions.len()
            );

            for suggestion in suggestions {
                println!("{} {}", suggestion.commit.cyan(), suggestion.message);
                println!(
                    "  {} Create a {} because: {}",
                    "→".yellow(),
                    suggestion.suggested_type.bold(),
                    suggestion.reason
                );
                if !suggestion.related.is_empty() {
                    println!(
                        "  {} Related: {}",
                        "↳".dimmed(),
                        suggestion.related.join(", ").dimmed()
                    );
                }
                println!();
            }
        }
    }

    Ok(())
}

struct Suggestion {
    commit: String,
    message: String,
    suggested_type: String,
    reason: String,
    related: Vec<String>,
}

fn analyze_commits(commits: &str, graph: &Graph) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();

    for line in commits.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() < 2 {
            continue;
        }

        let commit = parts[0];
        let message = parts[1].to_lowercase();

        // Check for decision keywords
        let has_decision = DECISION_KEYWORDS.iter().any(|k| message.contains(k));
        let has_arch = ARCHITECTURE_KEYWORDS.iter().any(|k| message.contains(k));
        let has_incident = INCIDENT_KEYWORDS.iter().any(|k| message.contains(k));

        // Find related records by searching the message
        let related = find_related_records(&message, graph);

        if has_decision && has_arch {
            // Likely an ADR
            suggestions.push(Suggestion {
                commit: commit.to_string(),
                message: parts[1].to_string(),
                suggested_type: "ADR".to_string(),
                reason: "architectural decision detected".to_string(),
                related,
            });
        } else if has_decision && !has_incident {
            // General decision
            suggestions.push(Suggestion {
                commit: commit.to_string(),
                message: parts[1].to_string(),
                suggested_type: "Decision".to_string(),
                reason: "decision language detected".to_string(),
                related,
            });
        } else if has_incident && is_significant_fix(&message) {
            // Potential incident
            suggestions.push(Suggestion {
                commit: commit.to_string(),
                message: parts[1].to_string(),
                suggested_type: "Incident".to_string(),
                reason: "significant fix or incident response".to_string(),
                related,
            });
        }
    }

    suggestions
}

fn find_related_records(message: &str, graph: &Graph) -> Vec<String> {
    let mut related = Vec::new();

    // Look for record IDs in the message
    let id_re = Regex::new(r"\b([A-Z]{3}-\d{3})\b").unwrap();
    for cap in id_re.captures_iter(message) {
        if let Some(id) = cap.get(1) {
            if graph.get(id.as_str()).is_some() {
                related.push(id.as_str().to_string());
            }
        }
    }

    // Search for matching records
    let search_results = graph.search(message, false);
    for record in search_results.iter().take(3) {
        if !related.contains(&record.id().to_string()) {
            related.push(record.id().to_string());
        }
    }

    related
}

fn is_significant_fix(message: &str) -> bool {
    let significant_indicators = [
        "critical",
        "urgent",
        "hotfix",
        "production",
        "outage",
        "security",
        "vulnerability",
        "crash",
        "data loss",
    ];
    significant_indicators.iter().any(|i| message.contains(i))
}
