//! Team management with hierarchy support

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Team information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Team {
    /// Display name (e.g., "Engineering")
    pub name: String,

    /// Team lead username
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lead: Option<String>,

    /// Parent team ID (for hierarchy)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// Team description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Team email (e.g., "engineering@piedpiper.com")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Team avatar URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

impl Team {
    /// Get avatar URL or generate a fallback
    #[allow(dead_code)]
    pub fn avatar(&self, team_id: &str) -> String {
        self.avatar_url.clone().unwrap_or_else(|| {
            let initials: String = self
                .name
                .split_whitespace()
                .filter_map(|word| word.chars().next())
                .take(2)
                .collect::<String>()
                .to_uppercase();

            let initials = if initials.is_empty() {
                team_id.chars().next().unwrap_or('T').to_string().to_uppercase()
            } else {
                initials
            };

            format!(
                "https://ui-avatars.com/api/?name={}&background=0f3460&color=fff&size=64&rounded=true",
                initials
            )
        })
    }
}

/// Teams configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TeamsConfig {
    /// Map of team_id -> Team
    #[serde(flatten)]
    pub teams: HashMap<String, Team>,
}

impl TeamsConfig {
    /// Check if a team exists
    pub fn exists(&self, team_id: &str) -> bool {
        self.teams.contains_key(team_id)
    }

    /// Get team info, returns None if not found
    pub fn get(&self, team_id: &str) -> Option<&Team> {
        self.teams.get(team_id)
    }

    /// Get root teams (no parent)
    pub fn root_teams(&self) -> Vec<(&String, &Team)> {
        self.teams
            .iter()
            .filter(|(_, t)| t.parent.is_none())
            .collect()
    }

    /// Get child teams of a parent
    pub fn children(&self, parent_id: &str) -> Vec<(&String, &Team)> {
        self.teams
            .iter()
            .filter(|(_, t)| t.parent.as_deref() == Some(parent_id))
            .collect()
    }

    /// Get the full hierarchy path for a team (e.g., ["engineering", "platform"])
    pub fn hierarchy_path(&self, team_id: &str) -> Vec<String> {
        let mut path = vec![];
        let mut current = team_id;

        while let Some(team) = self.get(current) {
            path.push(current.to_string());
            if let Some(ref parent) = team.parent {
                current = parent;
            } else {
                break;
            }
        }

        path.reverse();
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_exists() {
        let mut config = TeamsConfig::default();
        config.teams.insert(
            "engineering".to_string(),
            Team {
                name: "Engineering".to_string(),
                ..Default::default()
            },
        );

        assert!(config.exists("engineering"));
        assert!(!config.exists("marketing"));
    }

    #[test]
    fn test_team_hierarchy() {
        let mut config = TeamsConfig::default();
        config.teams.insert(
            "engineering".to_string(),
            Team {
                name: "Engineering".to_string(),
                ..Default::default()
            },
        );
        config.teams.insert(
            "platform".to_string(),
            Team {
                name: "Platform Team".to_string(),
                parent: Some("engineering".to_string()),
                ..Default::default()
            },
        );

        let path = config.hierarchy_path("platform");
        assert_eq!(path, vec!["engineering", "platform"]);
    }

    #[test]
    fn test_root_teams() {
        let mut config = TeamsConfig::default();
        config.teams.insert(
            "engineering".to_string(),
            Team {
                name: "Engineering".to_string(),
                ..Default::default()
            },
        );
        config.teams.insert(
            "platform".to_string(),
            Team {
                name: "Platform Team".to_string(),
                parent: Some("engineering".to_string()),
                ..Default::default()
            },
        );

        let roots = config.root_teams();
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].0, "engineering");
    }

    #[test]
    fn test_children() {
        let mut config = TeamsConfig::default();
        config.teams.insert(
            "engineering".to_string(),
            Team {
                name: "Engineering".to_string(),
                ..Default::default()
            },
        );
        config.teams.insert(
            "platform".to_string(),
            Team {
                name: "Platform Team".to_string(),
                parent: Some("engineering".to_string()),
                ..Default::default()
            },
        );

        let children = config.children("engineering");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].0, "platform");
    }

    #[test]
    fn test_team_avatar() {
        let team = Team {
            name: "Engineering".to_string(),
            ..Default::default()
        };
        assert!(team.avatar("engineering").contains("name=E"));
    }
}
