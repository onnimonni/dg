//! Team management CLI commands

use crate::models::teams::Team;
use crate::serve::config::DgConfig;
use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;

/// List all teams
pub fn list(docs_dir: &str, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let config = DgConfig::load(docs_path)?;
    let teams = config.teams_config();

    if teams.teams.is_empty() {
        println!("No teams found. Add teams to dg.toml [teams] section.");
        return Ok(());
    }

    match format {
        "json" => {
            let output: Vec<_> = teams
                .teams
                .iter()
                .map(|(id, team)| {
                    serde_json::json!({
                        "id": id,
                        "name": team.name,
                        "lead": team.lead,
                        "parent": team.parent,
                        "description": team.description,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            // Print hierarchically
            println!("{:<15} {:<25} {:<15} {}", "ID", "NAME", "LEAD", "PARENT");
            println!("{}", "-".repeat(65));

            // First show root teams, then their children
            let mut roots: Vec<_> = teams.root_teams();
            roots.sort_by(|a, b| a.0.cmp(b.0));

            fn print_team(
                teams: &crate::models::teams::TeamsConfig,
                id: &str,
                team: &Team,
                indent: usize,
            ) {
                let prefix = "  ".repeat(indent);
                let lead = team.lead.as_deref().unwrap_or("-");
                let parent = team.parent.as_deref().unwrap_or("-");
                println!(
                    "{}{:<width$} {:<25} {:<15} {}",
                    prefix,
                    id,
                    team.name,
                    lead,
                    parent,
                    width = 15 - indent * 2
                );

                // Print children
                let mut children: Vec<_> = teams.children(id);
                children.sort_by(|a, b| a.0.cmp(b.0));
                for (child_id, child) in children {
                    print_team(teams, child_id, child, indent + 1);
                }
            }

            for (id, team) in &roots {
                print_team(&teams, id, team, 0);
            }

            println!("\nTotal: {} teams", teams.teams.len());
        }
    }

    Ok(())
}

/// Show a single team's details
pub fn show(docs_dir: &str, team_id: &str, format: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let config = DgConfig::load(docs_path)?;
    let teams = config.teams_config();
    let users = config.users_config();

    let team = teams
        .get(team_id)
        .ok_or_else(|| anyhow!("Team not found: {}", team_id))?;

    // Find team members from users config
    let members: Vec<_> = users
        .users
        .iter()
        .filter(|(_, u)| u.teams.contains(&team_id.to_string()))
        .map(|(username, _)| username.clone())
        .collect();

    match format {
        "json" => {
            let output = serde_json::json!({
                "id": team_id,
                "name": team.name,
                "lead": team.lead,
                "parent": team.parent,
                "description": team.description,
                "email": team.email,
                "members": members,
                "hierarchy": teams.hierarchy_path(team_id),
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            println!("{}", team.name.bold());
            println!("ID: {}", team_id);
            println!();

            if let Some(desc) = &team.description {
                println!("  {}", desc);
                println!();
            }

            if let Some(lead) = &team.lead {
                println!("  Lead:   @{}", lead);
            }
            if let Some(parent) = &team.parent {
                println!("  Parent: {}", parent);
            }
            if let Some(email) = &team.email {
                println!("  Email:  {}", email);
            }

            // Show hierarchy
            let path = teams.hierarchy_path(team_id);
            if path.len() > 1 {
                println!("  Path:   {}", path.join(" → "));
            }

            // Show members
            if !members.is_empty() {
                println!();
                println!("  Members ({}):", members.len());
                for member in &members {
                    println!("    @{}", member);
                }
            }

            // Show sub-teams
            let children = teams.children(team_id);
            if !children.is_empty() {
                println!();
                println!("  Sub-teams ({}):", children.len());
                for (child_id, child) in children {
                    println!("    {} ({})", child_id, child.name);
                }
            }
        }
    }

    Ok(())
}

/// Create a new team
pub fn create(
    docs_dir: &str,
    team_id: &str,
    name: &str,
    lead: Option<&str>,
    parent: Option<&str>,
) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut config = DgConfig::load(docs_path)?;

    if config.teams.contains_key(team_id) {
        return Err(anyhow!("Team already exists: {}", team_id));
    }

    // Validate parent exists if specified
    if let Some(p) = parent {
        if !config.teams.contains_key(p) {
            return Err(anyhow!("Parent team not found: {}", p));
        }
    }

    // Validate lead exists if specified
    if let Some(l) = lead {
        if !config.users.contains_key(l) {
            println!(
                "{} Lead user '{}' not found in users config",
                "WARN".yellow(),
                l
            );
        }
    }

    let team = Team {
        name: name.to_string(),
        lead: lead.map(|s| s.to_string()),
        parent: parent.map(|s| s.to_string()),
        ..Default::default()
    };

    config.teams.insert(team_id.to_string(), team);
    config.save(docs_path)?;

    println!(
        "{} Created team: {} ({})",
        "OK".green().bold(),
        team_id,
        name
    );
    Ok(())
}

/// Add a member to a team
pub fn add_member(docs_dir: &str, team_id: &str, username: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut config = DgConfig::load(docs_path)?;

    // Validate team exists
    if !config.teams.contains_key(team_id) {
        return Err(anyhow!("Team not found: {}", team_id));
    }

    // Validate user exists
    let user = config
        .users
        .get_mut(username)
        .ok_or_else(|| anyhow!("User not found: {}", username))?;

    // Check if already a member
    if user.teams.contains(&team_id.to_string()) {
        return Err(anyhow!(
            "User {} is already a member of {}",
            username,
            team_id
        ));
    }

    user.teams.push(team_id.to_string());
    config.save(docs_path)?;

    println!(
        "{} Added @{} to team {}",
        "OK".green().bold(),
        username,
        team_id
    );
    Ok(())
}

/// Remove a member from a team
pub fn remove_member(docs_dir: &str, team_id: &str, username: &str) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut config = DgConfig::load(docs_path)?;

    // Validate team exists
    if !config.teams.contains_key(team_id) {
        return Err(anyhow!("Team not found: {}", team_id));
    }

    // Validate user exists
    let user = config
        .users
        .get_mut(username)
        .ok_or_else(|| anyhow!("User not found: {}", username))?;

    // Check if a member
    if !user.teams.contains(&team_id.to_string()) {
        return Err(anyhow!("User {} is not a member of {}", username, team_id));
    }

    user.teams.retain(|t| t != team_id);
    config.save(docs_path)?;

    println!(
        "{} Removed @{} from team {}",
        "OK".green().bold(),
        username,
        team_id
    );
    Ok(())
}

/// Import teams from GitHub organization
pub fn import_github(docs_dir: &str, org: &str, dry_run: bool) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    let mut config = DgConfig::load(docs_path)?;

    println!("Fetching teams from GitHub org: {}", org);

    // Use gh CLI to get org teams
    let output = std::process::Command::new("gh")
        .args(["api", &format!("orgs/{}/teams", org), "--paginate"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to fetch GitHub teams: {}", stderr));
    }

    let gh_teams: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;

    let mut added = 0;
    let mut skipped = 0;

    for gh_team in &gh_teams {
        let slug = gh_team["slug"].as_str().unwrap_or_default();
        if slug.is_empty() {
            continue;
        }

        if config.teams.contains_key(slug) {
            skipped += 1;
            if dry_run {
                println!("  {} {} (already exists)", "SKIP".yellow(), slug);
            }
            continue;
        }

        let team = Team {
            name: gh_team["name"].as_str().unwrap_or(slug).to_string(),
            description: gh_team["description"].as_str().map(|s| s.to_string()),
            ..Default::default()
        };

        if dry_run {
            println!("  {} {} ({})", "ADD".green(), slug, team.name);
        } else {
            config.teams.insert(slug.to_string(), team);
        }
        added += 1;
    }

    if !dry_run && added > 0 {
        config.save(docs_path)?;
    }

    println!();
    if dry_run {
        println!(
            "Dry run complete. Would add {} teams, skip {}.",
            added, skipped
        );
    } else {
        println!(
            "{} Imported {} teams, skipped {} existing",
            "OK".green().bold(),
            added,
            skipped
        );
    }

    Ok(())
}
