use crate::models::Graph;
use crate::serve::config::DgConfig;
use anyhow::Result;
use colored::Colorize;
use std::collections::HashSet;
use std::path::Path;
use std::process::Command;

pub fn run(
    docs_dir: &str,
    user: Option<&str>,
    all: bool,
    include_completed: bool,
    format: &str,
    include_github: bool,
) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let graph = Graph::load(docs_path)?;

    // Load config to get team memberships
    let config = DgConfig::load(docs_path).unwrap_or_default();

    // Determine which user(s) to filter by
    let target_users: HashSet<String> = if all {
        HashSet::new() // Empty set means show all
    } else {
        let username = user
            .map(|s| s.to_string())
            .or_else(|| std::env::var("USER").ok())
            .or_else(|| std::env::var("USERNAME").ok());

        match username {
            Some(u) => {
                let mut users = HashSet::new();
                users.insert(u.to_lowercase());
                users
            }
            None => {
                eprintln!(
                    "{}",
                    "Could not determine current user. Use --user or --all.".red()
                );
                return Ok(());
            }
        }
    };

    // Get teams the user belongs to (for team-assigned tasks)
    let user_teams: HashSet<String> = if !all && !target_users.is_empty() {
        let username = target_users.iter().next().unwrap();
        config
            .users
            .get(username)
            .map(|u| u.teams.iter().cloned().collect())
            .unwrap_or_default()
    } else {
        HashSet::new()
    };

    // Collect all action items
    let mut tasks: Vec<TaskItem> = Vec::new();

    for record in graph.all_records() {
        for (text, completed, owner) in record.extract_action_items() {
            // Skip completed if not requested
            if completed && !include_completed {
                continue;
            }

            // Filter by user/team if not showing all
            if !all {
                let owner_lower = owner.as_ref().map(|o| o.to_lowercase());
                let matches_user = owner_lower
                    .as_ref()
                    .map(|o| target_users.contains(o))
                    .unwrap_or(false);
                let matches_team = owner_lower
                    .as_ref()
                    .map(|o| user_teams.contains(o))
                    .unwrap_or(false);

                if !matches_user && !matches_team {
                    continue;
                }
            }

            tasks.push(TaskItem {
                record_id: record.id().to_string(),
                record_title: record.title().to_string(),
                text,
                completed,
                owner,
            });
        }
    }

    // Sort: incomplete first, then by record ID
    tasks.sort_by(|a, b| {
        a.completed
            .cmp(&b.completed)
            .then_with(|| a.record_id.cmp(&b.record_id))
    });

    // Fetch GitHub issues if enabled
    let mut github_issues: Vec<GitHubIssue> = Vec::new();
    if include_github {
        if let Some(issues) = fetch_github_issues(&target_users, all, include_completed) {
            github_issues = issues;
        }
    }

    if tasks.is_empty() && github_issues.is_empty() {
        if format != "json" {
            if all {
                println!("{}", "No action items found.".green());
            } else {
                println!("{}", "No action items assigned to you.".green());
            }
        } else {
            println!("[]");
        }
        return Ok(());
    }

    match format {
        "json" => {
            let mut output: Vec<serde_json::Value> = tasks
                .iter()
                .map(|t| {
                    serde_json::json!({
                        "source": "document",
                        "record_id": t.record_id,
                        "record_title": t.record_title,
                        "text": t.text,
                        "completed": t.completed,
                        "owner": t.owner,
                    })
                })
                .collect();

            // Add GitHub issues to JSON output
            for issue in &github_issues {
                output.push(serde_json::json!({
                    "source": "github",
                    "number": issue.number,
                    "title": issue.title,
                    "state": issue.state,
                    "url": issue.url,
                    "relationship": issue.relationship,
                }));
            }

            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            let incomplete_count = tasks.iter().filter(|t| !t.completed).count();
            let completed_count = tasks.iter().filter(|t| t.completed).count();
            let gh_open_count = github_issues.iter().filter(|i| i.state == "OPEN").count();
            let gh_closed_count = github_issues.iter().filter(|i| i.state != "OPEN").count();

            // Document action items
            if !tasks.is_empty() {
                if incomplete_count > 0 {
                    println!(
                        "{} {} incomplete action items:\n",
                        "📋 Documents".cyan().bold(),
                        incomplete_count
                    );
                } else {
                    println!("{}", "📋 Documents".cyan().bold());
                }

                let mut current_record = String::new();
                for task in &tasks {
                    if task.record_id != current_record {
                        if !current_record.is_empty() {
                            println!();
                        }
                        println!(
                            "{} {}",
                            task.record_id.cyan().bold(),
                            task.record_title.dimmed()
                        );
                        current_record = task.record_id.clone();
                    }

                    let checkbox = if task.completed {
                        "[✓]".green().to_string()
                    } else {
                        "[ ]".yellow().to_string()
                    };

                    let text = if task.completed {
                        task.text.dimmed().to_string()
                    } else {
                        task.text.clone()
                    };

                    println!("  {} {}", checkbox, text);
                }
            }

            // GitHub issues
            if !github_issues.is_empty() {
                if !tasks.is_empty() {
                    println!();
                }
                println!(
                    "{} {} open issues:\n",
                    "🐙 GitHub".magenta().bold(),
                    gh_open_count
                );

                for issue in &github_issues {
                    let status = if issue.state == "OPEN" {
                        "○".yellow().to_string()
                    } else {
                        "●".green().to_string()
                    };

                    let rel = match issue.relationship.as_str() {
                        "assignee" => "(assigned)".dimmed(),
                        "author" => "(author)".dimmed(),
                        "both" => "(author+assigned)".dimmed(),
                        _ => "".dimmed(),
                    };

                    let title = if issue.state == "OPEN" {
                        issue.title.clone()
                    } else {
                        issue.title.dimmed().to_string()
                    };

                    println!(
                        "  {} #{} {} {}",
                        status,
                        issue.number.to_string().cyan(),
                        title,
                        rel
                    );
                }
            }

            // Summary
            let total_open = incomplete_count + gh_open_count;
            let total_closed = completed_count + gh_closed_count;
            if include_completed && (total_closed > 0 || total_open > 0) {
                println!(
                    "\n{} {} open, {} closed",
                    "Summary:".bold(),
                    total_open.to_string().yellow(),
                    total_closed.to_string().green()
                );
            }
        }
    }

    Ok(())
}

struct TaskItem {
    record_id: String,
    record_title: String,
    text: String,
    completed: bool,
    #[allow(dead_code)]
    owner: Option<String>,
}

#[derive(Debug)]
struct GitHubIssue {
    number: u64,
    title: String,
    state: String,
    url: String,
    relationship: String, // "assignee", "author", or "both"
}

/// Fetch GitHub issues for the current user
fn fetch_github_issues(
    target_users: &HashSet<String>,
    all: bool,
    include_completed: bool,
) -> Option<Vec<GitHubIssue>> {
    // Check if gh CLI is available
    if Command::new("gh").arg("--version").output().is_err() {
        return None;
    }

    // Check if we're in a GitHub repo
    let repo_check = Command::new("gh")
        .args(["repo", "view", "--json", "nameWithOwner"])
        .output()
        .ok()?;

    if !repo_check.status.success() {
        return None; // Not a GitHub repo
    }

    let mut issues: Vec<GitHubIssue> = Vec::new();
    let mut seen_numbers: HashSet<u64> = HashSet::new();

    // Build state filter
    let state_filter = if include_completed { "all" } else { "open" };

    // Fetch issues assigned to user
    let user_filter = if all {
        String::new()
    } else if let Some(username) = target_users.iter().next() {
        format!("--assignee={}", username)
    } else {
        "--assignee=@me".to_string()
    };

    // Fetch assigned issues
    let mut args = vec![
        "issue",
        "list",
        "--state",
        state_filter,
        "--json",
        "number,title,state,url",
        "--limit",
        "50",
    ];
    if !user_filter.is_empty() {
        args.push(&user_filter);
    }

    if let Ok(output) = Command::new("gh").args(&args).output() {
        if output.status.success() {
            if let Ok(json) = serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout) {
                for issue in json {
                    let number = issue["number"].as_u64().unwrap_or(0);
                    if number > 0 && !seen_numbers.contains(&number) {
                        seen_numbers.insert(number);
                        issues.push(GitHubIssue {
                            number,
                            title: issue["title"].as_str().unwrap_or("").to_string(),
                            state: issue["state"].as_str().unwrap_or("OPEN").to_string(),
                            url: issue["url"].as_str().unwrap_or("").to_string(),
                            relationship: "assignee".to_string(),
                        });
                    }
                }
            }
        }
    }

    // Fetch issues authored by user
    let author_filter = if all {
        String::new()
    } else {
        "--author=@me".to_string()
    };

    let mut args = vec![
        "issue",
        "list",
        "--state",
        state_filter,
        "--json",
        "number,title,state,url",
        "--limit",
        "50",
    ];
    if !author_filter.is_empty() {
        args.push(&author_filter);
    }

    if let Ok(output) = Command::new("gh").args(&args).output() {
        if output.status.success() {
            if let Ok(json) = serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout) {
                for issue in json {
                    let number = issue["number"].as_u64().unwrap_or(0);
                    if number > 0 {
                        if seen_numbers.contains(&number) {
                            // Update existing issue to show both relationships
                            if let Some(existing) = issues.iter_mut().find(|i| i.number == number) {
                                existing.relationship = "both".to_string();
                            }
                        } else {
                            seen_numbers.insert(number);
                            issues.push(GitHubIssue {
                                number,
                                title: issue["title"].as_str().unwrap_or("").to_string(),
                                state: issue["state"].as_str().unwrap_or("OPEN").to_string(),
                                url: issue["url"].as_str().unwrap_or("").to_string(),
                                relationship: "author".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    // Sort by number
    issues.sort_by(|a, b| {
        // Open issues first
        let a_open = a.state == "OPEN";
        let b_open = b.state == "OPEN";
        b_open.cmp(&a_open).then_with(|| a.number.cmp(&b.number))
    });

    Some(issues)
}
