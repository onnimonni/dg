//! User management CLI commands

use crate::models::users::User;
use crate::serve::config::DgConfig;
use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;

/// List all users
pub fn list(docs_dir: &str, include_deprecated: bool, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let config = DgConfig::load(docs_path)?;
    let users = config.users_config();

    let mut entries: Vec<_> = users.users.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    // Filter deprecated if not included
    if !include_deprecated {
        entries.retain(|(_, u)| !u.is_deprecated());
    }

    if entries.is_empty() {
        println!("No users found. Add users to dg.toml [users] section.");
        return Ok(());
    }

    match format {
        "json" => {
            let output: Vec<_> = entries
                .iter()
                .map(|(username, user)| {
                    serde_json::json!({
                        "username": username,
                        "name": user.name,
                        "email": user.email,
                        "github": user.github,
                        "teams": user.teams,
                        "roles": user.roles,
                        "deprecated": user.is_deprecated(),
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            println!(
                "{:<15} {:<25} {:<20} {}",
                "USERNAME", "NAME", "TEAMS", "STATUS"
            );
            println!("{}", "-".repeat(70));

            for (username, user) in &entries {
                let name = user.name.as_deref().unwrap_or("-");
                let teams = if user.teams.is_empty() {
                    "-".to_string()
                } else {
                    user.teams.join(", ")
                };
                let status = if user.is_deprecated() {
                    "deprecated".dimmed().to_string()
                } else {
                    "active".green().to_string()
                };
                println!("{:<15} {:<25} {:<20} {}", username, name, teams, status);
            }

            println!("\nTotal: {} users", entries.len());
        }
    }

    Ok(())
}

/// Show a single user's details
pub fn show(docs_dir: &str, username: &str, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let config = DgConfig::load(docs_path)?;
    let users = config.users_config();

    let user = users
        .get(username)
        .ok_or_else(|| anyhow!("User not found: {}", username))?;

    match format {
        "json" => {
            let output = serde_json::json!({
                "username": username,
                "name": user.name,
                "email": user.email,
                "github": user.github,
                "avatar_url": user.avatar(username),
                "teams": user.teams,
                "roles": user.roles,
                "deprecated": user.is_deprecated(),
                "deprecated_date": user.deprecated_date,
                "deprecated_note": user.deprecated_note,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            println!("{}", username.bold());
            println!();

            if let Some(name) = &user.name {
                println!("  Name:   {}", name);
            }
            if let Some(email) = &user.email {
                println!("  Email:  {}", email);
            }
            if let Some(github) = &user.github {
                println!("  GitHub: https://github.com/{}", github);
            }

            if !user.teams.is_empty() {
                println!("  Teams:  {}", user.teams.join(", "));
            }
            if !user.roles.is_empty() {
                println!("  Roles:  {}", user.roles.join(", "));
            }

            if user.is_deprecated() {
                println!();
                println!("  {} {}", "Status:".yellow(), "DEPRECATED".red());
                if let Some(date) = &user.deprecated_date {
                    println!("  Left:   {}", date);
                }
                if let Some(note) = &user.deprecated_note {
                    println!("  Note:   {}", note);
                }
            }
        }
    }

    Ok(())
}

/// Add a new user
pub fn add(
    docs_dir: &str,
    username: &str,
    name: Option<&str>,
    email: Option<&str>,
    github: Option<&str>,
    teams: Option<Vec<String>>,
) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut config = DgConfig::load(docs_path)?;

    if config.users.contains_key(username) {
        return Err(anyhow!("User already exists: {}", username));
    }

    let user = User {
        name: name.map(|s| s.to_string()),
        email: email.map(|s| s.to_string()),
        github: github.map(|s| s.to_string()),
        teams: teams.unwrap_or_default(),
        ..Default::default()
    };

    config.users.insert(username.to_string(), user);
    config.save(docs_path)?;

    println!("{} Added user: {}", "OK".green().bold(), username);
    Ok(())
}

/// Deprecate a user
pub fn deprecate(docs_dir: &str, username: &str, note: Option<&str>) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut config = DgConfig::load(docs_path)?;

    let user = config
        .users
        .get_mut(username)
        .ok_or_else(|| anyhow!("User not found: {}", username))?;

    if user.is_deprecated() {
        return Err(anyhow!("User is already deprecated: {}", username));
    }

    user.status = Some("deprecated".to_string());
    user.deprecated_date = Some(chrono::Local::now().format("%Y-%m-%d").to_string());
    if let Some(n) = note {
        user.deprecated_note = Some(n.to_string());
    }

    config.save(docs_path)?;

    println!("{} Deprecated user: {}", "OK".green().bold(), username);
    Ok(())
}

/// Import users from GitHub organization
pub fn import_github(docs_dir: &str, org: &str, dry_run: bool) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut config = DgConfig::load(docs_path)?;

    println!("Fetching members from GitHub org: {}", org);

    // Use gh CLI to get org members
    let output = std::process::Command::new("gh")
        .args(["api", &format!("orgs/{}/members", org), "--paginate"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to fetch GitHub members: {}", stderr));
    }

    let members: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;

    let mut added = 0;
    let mut skipped = 0;

    for member in &members {
        let login = member["login"].as_str().unwrap_or_default();
        if login.is_empty() {
            continue;
        }

        if config.users.contains_key(login) {
            skipped += 1;
            if dry_run {
                println!("  {} {} (already exists)", "SKIP".yellow(), login);
            }
            continue;
        }

        // Fetch user details
        let user_output = std::process::Command::new("gh")
            .args(["api", &format!("users/{}", login)])
            .output()?;

        let user_data: serde_json::Value = if user_output.status.success() {
            serde_json::from_slice(&user_output.stdout)?
        } else {
            serde_json::json!({})
        };

        let user = User {
            name: user_data["name"].as_str().map(|s| s.to_string()),
            email: user_data["email"].as_str().map(|s| s.to_string()),
            github: Some(login.to_string()),
            avatar_url: user_data["avatar_url"].as_str().map(|s| s.to_string()),
            ..Default::default()
        };

        if dry_run {
            println!(
                "  {} {} ({})",
                "ADD".green(),
                login,
                user.name.as_deref().unwrap_or("no name")
            );
        } else {
            config.users.insert(login.to_string(), user);
        }
        added += 1;
    }

    // Fetch team membership and assign users to teams
    println!("\nFetching team memberships...");
    let teams_output = std::process::Command::new("gh")
        .args(["api", &format!("orgs/{}/teams", org), "--paginate"])
        .output()?;

    if teams_output.status.success() {
        let gh_teams: Vec<serde_json::Value> = serde_json::from_slice(&teams_output.stdout)?;

        for gh_team in &gh_teams {
            let slug = gh_team["slug"].as_str().unwrap_or_default();
            if slug.is_empty() {
                continue;
            }

            // Get team members
            let members_output = std::process::Command::new("gh")
                .args([
                    "api",
                    &format!("orgs/{}/teams/{}/members", org, slug),
                    "--paginate",
                ])
                .output()?;

            if members_output.status.success() {
                let team_members: Vec<serde_json::Value> =
                    serde_json::from_slice(&members_output.stdout)?;

                for tm in &team_members {
                    let login = tm["login"].as_str().unwrap_or_default();
                    if let Some(user) = config.users.get_mut(login) {
                        if !user.teams.contains(&slug.to_string()) {
                            if dry_run {
                                println!("  {} @{} -> {}", "TEAM".cyan(), login, slug);
                            } else {
                                user.teams.push(slug.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    if !dry_run && added > 0 {
        config.save(docs_path)?;
    }

    println!();
    if dry_run {
        println!(
            "Dry run complete. Would add {} users, skip {}.",
            added, skipped
        );
    } else {
        println!(
            "{} Imported {} users, skipped {} existing",
            "OK".green().bold(),
            added,
            skipped
        );
    }

    Ok(())
}
