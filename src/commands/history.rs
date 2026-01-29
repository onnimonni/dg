//! History command - view historical configurations from git

use crate::git::GitHistory;
use anyhow::Result;
use chrono::NaiveDate;
use std::path::Path;

/// Show configuration history or config at a specific date
pub fn config(docs_dir: &str, at: Option<&str>, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let history = GitHistory::new(docs_path)?;

    if let Some(date_str) = at {
        // Show config at specific date
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| anyhow::anyhow!("Invalid date format. Use YYYY-MM-DD"))?;

        let config = history.config_at_date(&date)?;

        if format == "json" {
            println!("{}", serde_json::to_string_pretty(&config)?);
        } else {
            println!("Configuration as of {}", date);
            println!();

            // Show users
            if !config.users.is_empty() {
                println!("Users:");
                for (username, user) in &config.users {
                    let status = if user.is_deprecated() {
                        " (deprecated)"
                    } else {
                        ""
                    };
                    let name = user.name.as_deref().unwrap_or(username);
                    let teams = if user.teams.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", user.teams.join(", "))
                    };
                    println!("  @{} - {}{}{}", username, name, teams, status);
                }
                println!();
            }

            // Show teams
            if !config.teams.is_empty() {
                println!("Teams:");
                for (team_id, team) in &config.teams {
                    let lead = team
                        .lead
                        .as_ref()
                        .map(|l| format!(" (lead: @{})", l))
                        .unwrap_or_default();
                    let parent = team
                        .parent
                        .as_ref()
                        .map(|p| format!(" <- {}", p))
                        .unwrap_or_default();
                    println!("  {} - {}{}{}", team_id, team.name, lead, parent);
                }
            }
        }
    } else {
        // Show all config changes
        let snapshots = history.config_commits()?;

        if snapshots.is_empty() {
            println!("No configuration history found.");
            println!("Make sure dg.toml is tracked in git.");
            return Ok(());
        }

        if format == "json" {
            let json: Vec<_> = snapshots
                .iter()
                .map(|s| {
                    serde_json::json!({
                        "commit": s.commit,
                        "date": s.date.to_rfc3339(),
                        "message": s.message,
                        "users": s.config.users.len(),
                        "teams": s.config.teams.len(),
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json)?);
        } else {
            println!(
                "{:<10} {:<12} {:<8} {:<8} {}",
                "Commit", "Date", "Users", "Teams", "Message"
            );
            println!("{}", "-".repeat(70));

            for snapshot in &snapshots {
                println!(
                    "{:<10} {:<12} {:<8} {:<8} {}",
                    snapshot.commit,
                    snapshot.date.format("%Y-%m-%d"),
                    snapshot.config.users.len(),
                    snapshot.config.teams.len(),
                    truncate(&snapshot.message, 30)
                );
            }

            println!();
            println!("Use --at YYYY-MM-DD to see config at a specific date");
        }
    }

    Ok(())
}

/// Show team membership history
pub fn team(docs_dir: &str, team_id: &str, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let history = GitHistory::new(docs_path)?;

    let snapshots = history.team_history(team_id)?;

    if snapshots.is_empty() {
        println!("No history found for team '{}'.", team_id);
        return Ok(());
    }

    if format == "json" {
        let json: Vec<_> = snapshots
            .iter()
            .map(|s| {
                serde_json::json!({
                    "commit": s.commit,
                    "date": s.date.to_rfc3339(),
                    "message": s.message,
                    "members": s.members,
                    "joined": s.joined,
                    "left": s.left,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        println!("Team '{}' History", team_id);
        println!("{}", "=".repeat(50));
        println!();

        for snapshot in &snapshots {
            println!(
                "{} ({}) - {}",
                snapshot.date.format("%Y-%m-%d"),
                snapshot.commit,
                snapshot.message
            );

            if !snapshot.joined.is_empty() {
                println!(
                    "  + Joined: {}",
                    snapshot
                        .joined
                        .iter()
                        .map(|u| format!("@{}", u))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            if !snapshot.left.is_empty() {
                println!(
                    "  - Left: {}",
                    snapshot
                        .left
                        .iter()
                        .map(|u| format!("@{}", u))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            println!(
                "  Members: {}",
                snapshot
                    .members
                    .iter()
                    .map(|u| format!("@{}", u))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            println!();
        }

        // Show all-time members
        let all_time = history.all_time_members(team_id)?;
        println!("All-time members: {}", all_time.len());
        for member in all_time {
            println!("  @{}", member);
        }
    }

    Ok(())
}

/// Show user tenure across teams
pub fn user(docs_dir: &str, username: &str, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let history = GitHistory::new(docs_path)?;

    // Get all teams this user was ever part of
    let snapshots = history.config_commits()?;
    let mut teams_found: Vec<String> = Vec::new();

    for snapshot in &snapshots {
        for (_, user) in &snapshot.config.users {
            for team in &user.teams {
                if !teams_found.contains(team) {
                    teams_found.push(team.clone());
                }
            }
        }
        // Also check for the specific user
        if let Some(user) = snapshot.config.users.get(username) {
            for team in &user.teams {
                if !teams_found.contains(team) {
                    teams_found.push(team.clone());
                }
            }
        }
    }

    if format == "json" {
        let mut tenures = Vec::new();
        for team_id in &teams_found {
            if let Ok(periods) = history.user_tenure(username, team_id) {
                if !periods.is_empty() {
                    tenures.push(serde_json::json!({
                        "team": team_id,
                        "periods": periods.iter().map(|p| {
                            serde_json::json!({
                                "start": p.start.to_rfc3339(),
                                "end": p.end.map(|e| e.to_rfc3339()),
                            })
                        }).collect::<Vec<_>>(),
                    }));
                }
            }
        }
        println!("{}", serde_json::to_string_pretty(&tenures)?);
    } else {
        println!("@{} Team History", username);
        println!("{}", "=".repeat(50));
        println!();

        let mut found_any = false;
        for team_id in &teams_found {
            if let Ok(periods) = history.user_tenure(username, team_id) {
                if !periods.is_empty() {
                    found_any = true;
                    println!("Team: {}", team_id);
                    for period in periods {
                        let end = period
                            .end
                            .map(|e| e.format("%Y-%m-%d").to_string())
                            .unwrap_or_else(|| "present".to_string());
                        println!("  {} to {}", period.start.format("%Y-%m-%d"), end);
                    }
                    println!();
                }
            }
        }

        if !found_any {
            println!("No team history found for @{}", username);
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
