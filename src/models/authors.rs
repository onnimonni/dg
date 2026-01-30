//! Author information management with GitHub integration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Author information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorInfo {
    /// Display name (e.g., "Onni Hakala")
    pub name: Option<String>,
    /// Email address
    pub email: Option<String>,
    /// Avatar URL (typically from GitHub)
    pub avatar_url: Option<String>,
    /// GitHub username (for fetching info)
    pub github: Option<String>,
    /// Override the display username in docs
    pub username: Option<String>,
}

impl AuthorInfo {
    /// Get display name or fall back to username
    pub fn display_name(&self, username: &str) -> String {
        self.name.clone().unwrap_or_else(|| username.to_string())
    }

    /// Get initials (e.g., "OH" for "Onni Hakala")
    pub fn initials(&self, username: &str) -> String {
        let name = self.display_name(username);
        name.split_whitespace()
            .filter_map(|word| word.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase()
    }

    /// Get avatar URL or generate a fallback
    pub fn avatar(&self, username: &str) -> String {
        self.avatar_url.clone().unwrap_or_else(|| {
            // If we have a GitHub username, use their avatar
            if let Some(ref gh) = self.github {
                format!("https://github.com/{}.png?size=64", gh)
            } else {
                // Use UI Avatars as fallback
                let initials = self.initials(username);
                format!(
                    "https://ui-avatars.com/api/?name={}&background=007c43&color=fff&size=64",
                    initials
                )
            }
        })
    }
}

/// Authors configuration and cache
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorsConfig {
    /// Manual author overrides (username -> info)
    #[serde(default)]
    pub authors: HashMap<String, AuthorInfo>,

    /// Whether to use GitHub avatars for unknown users (default: true)
    #[serde(skip)]
    pub github_avatars: bool,
}

impl AuthorsConfig {
    /// Create with github_avatars setting
    pub fn with_github_avatars(authors: HashMap<String, AuthorInfo>, github_avatars: bool) -> Self {
        Self {
            authors,
            github_avatars,
        }
    }

    /// Get author info, with defaults for unknown authors
    pub fn get(&self, username: &str) -> AuthorInfo {
        self.authors.get(username).cloned().unwrap_or_else(|| {
            if self.github_avatars {
                // Default: assume username is a GitHub username
                AuthorInfo {
                    github: Some(username.to_string()),
                    ..Default::default()
                }
            } else {
                // Use initials-only (no GitHub)
                AuthorInfo::default()
            }
        })
    }

    /// Resolve author info for display
    pub fn resolve(&self, username: &str) -> ResolvedAuthor {
        let info = self.get(username);
        ResolvedAuthor {
            username: info
                .username
                .clone()
                .unwrap_or_else(|| username.to_string()),
            name: info.display_name(username),
            initials: info.initials(username),
            avatar_url: info.avatar(username),
            email: info.email.clone(),
        }
    }
}

/// Resolved author info ready for display
#[derive(Debug, Clone, Serialize)]
pub struct ResolvedAuthor {
    pub username: String,
    pub name: String,
    pub initials: String,
    pub avatar_url: String,
    pub email: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initials() {
        let info = AuthorInfo {
            name: Some("Onni Hakala".to_string()),
            ..Default::default()
        };
        assert_eq!(info.initials("onni"), "OH");
    }

    #[test]
    fn test_initials_single_name() {
        let info = AuthorInfo {
            name: Some("Richard".to_string()),
            ..Default::default()
        };
        assert_eq!(info.initials("richard"), "R");
    }

    #[test]
    fn test_initials_fallback() {
        let info = AuthorInfo::default();
        assert_eq!(info.initials("jared"), "J");
    }

    #[test]
    fn test_avatar_github() {
        let info = AuthorInfo {
            github: Some("onnimonni".to_string()),
            ..Default::default()
        };
        assert!(info.avatar("onni").contains("github.com/onnimonni"));
    }
}
