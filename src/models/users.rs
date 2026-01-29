//! User management with status tracking

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// User information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    /// Display name (e.g., "Richard Hendricks")
    pub name: Option<String>,

    /// Email address
    pub email: Option<String>,

    /// GitHub username (for avatar and linking)
    pub github: Option<String>,

    /// Custom avatar URL (overrides GitHub avatar)
    pub avatar_url: Option<String>,

    /// User status (active or deprecated)
    #[serde(default, skip_serializing_if = "is_active")]
    pub status: Option<String>,

    /// Date user was deprecated
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_date: Option<String>,

    /// Note about deprecation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_note: Option<String>,

    /// Teams this user belongs to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<String>,

    /// Roles this user has
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
}

fn is_active(status: &Option<String>) -> bool {
    status.as_deref() != Some("deprecated")
}

impl User {
    /// Check if user is deprecated
    pub fn is_deprecated(&self) -> bool {
        self.status.as_deref() == Some("deprecated")
    }

    /// Get display name or fall back to username
    pub fn display_name(&self, username: &str) -> String {
        self.name.clone().unwrap_or_else(|| username.to_string())
    }

    /// Get initials (e.g., "RH" for "Richard Hendricks")
    pub fn initials(&self, username: &str) -> String {
        let name = self.display_name(username);
        name.split_whitespace()
            .filter_map(|word| word.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase()
    }

    /// Check if user is an LLM
    pub fn is_llm(&self) -> bool {
        self.roles.contains(&"llm".to_string())
    }

    /// Get avatar URL or generate a fallback
    pub fn avatar(&self, username: &str) -> String {
        self.avatar_url.clone().unwrap_or_else(|| {
            // LLMs get their provider's logo automatically
            if self.is_llm() {
                if let Some(url) = llm_avatar(username) {
                    return url.to_string();
                }
            }
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

/// Get default avatar URL for known LLM providers
fn llm_avatar(username: &str) -> Option<&'static str> {
    match username.to_lowercase().as_str() {
        "claude" => Some("https://www.anthropic.com/images/icons/apple-touch-icon.png"),
        "gemini" => Some(
            "https://www.gstatic.com/lamda/images/gemini_sparkle_v002_d4735304ff6292a690345.svg",
        ),
        "grok" => Some("https://x.ai/favicon.ico"),
        "copilot" | "github-copilot" => {
            Some("https://github.githubassets.com/assets/copilot-logo-8cddb06ee4d2.svg")
        }
        "chatgpt" | "openai" | "gpt" => {
            Some("https://cdn.oaistatic.com/assets/apple-touch-icon-mz9nytnj.webp")
        }
        "deepseek" => Some("https://www.deepseek.com/favicon.ico"),
        "llama" | "meta" => Some("https://llama.meta.com/favicon.ico"),
        "mistral" => Some("https://mistral.ai/favicon.ico"),
        "perplexity" => Some("https://www.perplexity.ai/favicon.ico"),
        _ => None,
    }
}

/// Users configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsersConfig {
    /// Map of username -> User
    #[serde(flatten)]
    pub users: HashMap<String, User>,
}

impl UsersConfig {
    /// Check if a username exists
    pub fn exists(&self, username: &str) -> bool {
        self.users.contains_key(username)
    }

    /// Get user info, returns None if not found
    pub fn get(&self, username: &str) -> Option<&User> {
        self.users.get(username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_display_name() {
        let user = User {
            name: Some("Richard Hendricks".to_string()),
            ..Default::default()
        };
        assert_eq!(user.display_name("richard"), "Richard Hendricks");
    }

    #[test]
    fn test_user_display_name_fallback() {
        let user = User::default();
        assert_eq!(user.display_name("richard"), "richard");
    }

    #[test]
    fn test_user_initials() {
        let user = User {
            name: Some("Richard Hendricks".to_string()),
            ..Default::default()
        };
        assert_eq!(user.initials("richard"), "RH");
    }

    #[test]
    fn test_user_is_deprecated() {
        let active = User::default();
        assert!(!active.is_deprecated());

        let deprecated = User {
            status: Some("deprecated".to_string()),
            ..Default::default()
        };
        assert!(deprecated.is_deprecated());
    }

    #[test]
    fn test_users_config_exists() {
        let mut config = UsersConfig::default();
        config.users.insert("richard".to_string(), User::default());

        assert!(config.exists("richard"));
        assert!(!config.exists("jared"));
    }

    #[test]
    fn test_avatar_github() {
        let user = User {
            github: Some("richardhendricks".to_string()),
            ..Default::default()
        };
        assert!(user
            .avatar("richard")
            .contains("github.com/richardhendricks"));
    }

    #[test]
    fn test_avatar_llm_auto() {
        // LLM users get automatic avatars based on username
        let claude = User {
            roles: vec!["llm".to_string()],
            ..Default::default()
        };
        assert!(claude.avatar("claude").contains("anthropic.com"));

        let gemini = User {
            roles: vec!["llm".to_string()],
            ..Default::default()
        };
        assert!(gemini.avatar("gemini").contains("gstatic.com"));

        // Unknown LLM falls back to initials
        let unknown_llm = User {
            name: Some("Custom Bot".to_string()),
            roles: vec!["llm".to_string()],
            ..Default::default()
        };
        assert!(unknown_llm.avatar("custombot").contains("ui-avatars.com"));
    }
}
