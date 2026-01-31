//! Git history traversal for dg.toml configurations
//!
//! Allows viewing team and user configurations at any point in git history.

use crate::serve::config::DgConfig;
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

/// A snapshot of configuration at a specific commit
#[derive(Debug, Clone)]
pub struct ConfigSnapshot {
    /// Git commit hash (short)
    pub commit: String,
    /// Commit date
    pub date: DateTime<Utc>,
    /// Commit message (first line)
    pub message: String,
    /// The configuration at this commit
    pub config: DgConfig,
}

/// Git history traversal for configurations
pub struct GitHistory {
    /// Path to the repository root
    repo_path: std::path::PathBuf,
    /// Relative path to dg.toml from repo root
    config_rel_path: String,
}

impl GitHistory {
    /// Create a new GitHistory for the given docs directory
    pub fn new(docs_dir: &Path) -> Result<Self> {
        // Find the git repo root
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(docs_dir)
            .output()
            .context("Failed to find git repository")?;

        if !output.status.success() {
            anyhow::bail!("Not a git repository");
        }

        let repo_path = String::from_utf8(output.stdout)?.trim().to_string();
        let repo_path = std::path::PathBuf::from(repo_path);

        // Calculate relative path from repo root to dg.toml
        let config_path = docs_dir.join("dg.toml");
        let config_rel_path = config_path
            .strip_prefix(&repo_path)
            .unwrap_or(Path::new("docs/dg.toml"))
            .to_string_lossy()
            .to_string();

        Ok(Self {
            repo_path,
            config_rel_path,
        })
    }

    /// Get all commits that modified dg.toml
    pub fn config_commits(&self) -> Result<Vec<ConfigSnapshot>> {
        let output = Command::new("git")
            .args([
                "log",
                "--follow",
                "--format=%H|%aI|%s",
                "--",
                &self.config_rel_path,
            ])
            .current_dir(&self.repo_path)
            .output()
            .context("Failed to get git log")?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let stdout = String::from_utf8(output.stdout)?;
        let mut snapshots = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.splitn(3, '|').collect();
            if parts.len() < 3 {
                continue;
            }

            let commit = parts[0].to_string();
            let date_str = parts[1];
            let message = parts[2].to_string();

            // Parse date
            let date = DateTime::parse_from_rfc3339(date_str)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            // Get config at this commit
            if let Ok(config) = self.config_at_commit(&commit) {
                snapshots.push(ConfigSnapshot {
                    commit: commit[..7.min(commit.len())].to_string(),
                    date,
                    message,
                    config,
                });
            }
        }

        Ok(snapshots)
    }

    /// Get config at a specific commit
    pub fn config_at_commit(&self, commit: &str) -> Result<DgConfig> {
        let file_spec = format!("{}:{}", commit, self.config_rel_path);
        let output = Command::new("git")
            .args(["show", &file_spec])
            .current_dir(&self.repo_path)
            .output()
            .context("Failed to get file at commit")?;

        if !output.status.success() {
            anyhow::bail!("Config not found at commit {}", commit);
        }

        let content = String::from_utf8(output.stdout)?;
        let config: DgConfig =
            toml::from_str(&content).context("Failed to parse config at commit")?;

        Ok(config)
    }

    /// Get config as it was on a specific date
    pub fn config_at_date(&self, date: &NaiveDate) -> Result<DgConfig> {
        // Find the commit that was active on this date
        let date_str = date.format("%Y-%m-%d").to_string();
        let output = Command::new("git")
            .args([
                "log",
                "-1",
                "--format=%H",
                &format!("--until={}", date_str),
                "--",
                &self.config_rel_path,
            ])
            .current_dir(&self.repo_path)
            .output()
            .context("Failed to get commit at date")?;

        if !output.status.success() || output.stdout.is_empty() {
            anyhow::bail!("No config found for date {}", date_str);
        }

        let commit = String::from_utf8(output.stdout)?.trim().to_string();
        if commit.is_empty() {
            anyhow::bail!("No config found for date {}", date_str);
        }

        self.config_at_commit(&commit)
    }

    /// Get team membership changes over time
    pub fn team_history(&self, team_id: &str) -> Result<Vec<TeamSnapshot>> {
        let snapshots = self.config_commits()?;
        let mut history = Vec::new();
        let mut prev_members: Option<Vec<String>> = None;

        for snapshot in snapshots.iter().rev() {
            // Get members of this team at this commit
            let members: Vec<String> = snapshot
                .config
                .users
                .iter()
                .filter(|(_, u)| u.teams.contains(&team_id.to_string()))
                .map(|(username, _)| username.clone())
                .collect();

            // Check if membership changed
            let changed = match &prev_members {
                Some(prev) => {
                    let mut prev_sorted = prev.clone();
                    let mut curr_sorted = members.clone();
                    prev_sorted.sort();
                    curr_sorted.sort();
                    prev_sorted != curr_sorted
                }
                None => true,
            };

            if changed {
                // Calculate who joined and left
                let (joined, left) = if let Some(ref prev) = prev_members {
                    let joined: Vec<String> = members
                        .iter()
                        .filter(|m| !prev.contains(m))
                        .cloned()
                        .collect();
                    let left: Vec<String> = prev
                        .iter()
                        .filter(|m| !members.contains(m))
                        .cloned()
                        .collect();
                    (joined, left)
                } else {
                    (members.clone(), vec![])
                };

                history.push(TeamSnapshot {
                    commit: snapshot.commit.clone(),
                    date: snapshot.date,
                    message: snapshot.message.clone(),
                    members: members.clone(),
                    joined,
                    left,
                });
            }

            prev_members = Some(members);
        }

        // Reverse so newest is first
        history.reverse();
        Ok(history)
    }

    /// Get all users who were ever members (including departed)
    pub fn all_time_members(&self, team_id: &str) -> Result<Vec<String>> {
        let history = self.team_history(team_id)?;
        let mut all_members: Vec<String> = history
            .iter()
            .flat_map(|s| s.members.iter().cloned())
            .collect();

        all_members.sort();
        all_members.dedup();
        Ok(all_members)
    }

    /// Get user tenure on a team
    pub fn user_tenure(&self, username: &str, team_id: &str) -> Result<Vec<TenurePeriod>> {
        let history = self.team_history(team_id)?;
        let mut periods = Vec::new();
        let mut start_date: Option<DateTime<Utc>> = None;

        // Process in chronological order (oldest first)
        for snapshot in history.iter().rev() {
            let is_member = snapshot.members.contains(&username.to_string());

            match (start_date, is_member) {
                (None, true) => {
                    // User joined
                    start_date = Some(snapshot.date);
                }
                (Some(start), false) => {
                    // User left
                    periods.push(TenurePeriod {
                        start,
                        end: Some(snapshot.date),
                    });
                    start_date = None;
                }
                _ => {}
            }
        }

        // If still a member, add open-ended period
        if let Some(start) = start_date {
            periods.push(TenurePeriod { start, end: None });
        }

        Ok(periods)
    }
}

/// Snapshot of a team at a point in time
#[derive(Debug, Clone)]
pub struct TeamSnapshot {
    /// Git commit hash (short)
    pub commit: String,
    /// Commit date
    pub date: DateTime<Utc>,
    /// Commit message
    pub message: String,
    /// Members at this point
    pub members: Vec<String>,
    /// Who joined in this change
    pub joined: Vec<String>,
    /// Who left in this change
    pub left: Vec<String>,
}

/// A period of time a user was on a team
#[derive(Debug, Clone)]
pub struct TenurePeriod {
    /// When they joined
    pub start: DateTime<Utc>,
    /// When they left (None if still active)
    pub end: Option<DateTime<Utc>>,
}

/// Cache for historical configurations
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct HistoryCache {
    /// Cached configs by date (YYYY-MM-DD -> Config)
    configs: HashMap<String, DgConfig>,
}

#[allow(dead_code)]
impl HistoryCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get or load config for a date
    pub fn get_or_load(&mut self, history: &GitHistory, date: &NaiveDate) -> Option<&DgConfig> {
        let key = date.format("%Y-%m-%d").to_string();

        if !self.configs.contains_key(&key) {
            if let Ok(config) = history.config_at_date(date) {
                self.configs.insert(key.clone(), config);
            }
        }

        self.configs.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_git_repo() -> Result<TempDir> {
        let dir = TempDir::new()?;

        Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .output()?;

        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(dir.path())
            .output()?;

        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(dir.path())
            .output()?;

        // Create docs dir
        std::fs::create_dir_all(dir.path().join("docs"))?;

        Ok(dir)
    }

    #[test]
    fn test_git_history_new() {
        let dir = setup_git_repo().unwrap();
        let docs_dir = dir.path().join("docs");

        // Create initial dg.toml
        let config = r#"
[site]
title = "Test"

[users.alice]
name = "Alice"
teams = ["engineering"]

[teams.engineering]
name = "Engineering"
"#;
        std::fs::write(docs_dir.join("dg.toml"), config).unwrap();

        Command::new("git")
            .args(["add", "."])
            .current_dir(dir.path())
            .output()
            .unwrap();

        Command::new("git")
            .args(["commit", "-m", "Initial config"])
            .current_dir(dir.path())
            .output()
            .unwrap();

        let history = GitHistory::new(&docs_dir).unwrap();
        assert!(history.config_rel_path.contains("dg.toml"));
    }
}
