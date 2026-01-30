use crate::models::authors::{AuthorInfo, AuthorsConfig};
use crate::models::teams::{Team, TeamsConfig};
use crate::models::users::{User, UsersConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Root configuration loaded from dg.toml
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DgConfig {
    /// Site branding settings
    #[serde(default)]
    pub site: SiteConfig,

    /// Author profiles (legacy, prefer users)
    #[serde(default)]
    pub authors: HashMap<String, AuthorInfo>,

    /// User profiles
    #[serde(default)]
    pub users: HashMap<String, User>,

    /// Team definitions
    #[serde(default)]
    pub teams: HashMap<String, Team>,
}

impl DgConfig {
    /// Load config from dg.toml (in docs_dir or parent directory)
    pub fn load(docs_dir: &Path) -> Result<Self> {
        // First try docs_dir/dg.toml
        let config_path = docs_dir.join("dg.toml");
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: DgConfig = toml::from_str(&content)?;
            return Ok(config);
        }

        // Then try parent directory (project root)
        if let Some(parent) = docs_dir.parent() {
            let parent_config = parent.join("dg.toml");
            if parent_config.exists() {
                let content = fs::read_to_string(&parent_config)?;
                let config: DgConfig = toml::from_str(&content)?;
                return Ok(config);
            }
        }

        // Try legacy .site.yaml for backwards compatibility
        let legacy_path = docs_dir.join(".site.yaml");
        if legacy_path.exists() {
            let content = fs::read_to_string(&legacy_path)?;
            let site: SiteConfig = serde_yaml::from_str(&content)?;
            return Ok(DgConfig {
                site,
                authors: HashMap::new(),
                users: HashMap::new(),
                teams: HashMap::new(),
            });
        }

        Ok(Self::default())
    }

    /// Get AuthorsConfig from the loaded authors map
    pub fn authors_config(&self) -> AuthorsConfig {
        AuthorsConfig::with_github_avatars(self.authors.clone(), self.site.github_avatars)
    }

    /// Get UsersConfig from the loaded users map
    pub fn users_config(&self) -> UsersConfig {
        UsersConfig {
            users: self.users.clone(),
        }
    }

    /// Get TeamsConfig from the loaded teams map
    pub fn teams_config(&self) -> TeamsConfig {
        TeamsConfig {
            teams: self.teams.clone(),
        }
    }

    /// Get the path to the config file
    pub fn config_path(docs_dir: &Path) -> std::path::PathBuf {
        docs_dir.join("dg.toml")
    }

    /// Save config back to dg.toml
    pub fn save(&self, docs_dir: &Path) -> Result<()> {
        let config_path = Self::config_path(docs_dir);
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }
}

/// Site configuration for branding
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SiteConfig {
    /// Site title (default: "Decision Graph")
    #[serde(default = "default_title")]
    pub title: String,

    /// Site description
    #[serde(default)]
    pub description: String,

    /// Path to logo image (relative to docs dir)
    #[serde(default)]
    pub logo: Option<String>,

    /// Primary color (hex)
    #[serde(default = "default_primary_color")]
    pub primary_color: String,

    /// Accent color (hex)
    #[serde(default = "default_accent_color")]
    pub accent_color: String,

    /// Custom CSS to inject
    #[serde(default)]
    pub custom_css: Option<String>,

    /// Footer text
    #[serde(default)]
    pub footer: Option<String>,

    /// Enable quick preview popups on record ID hover (default: true)
    #[serde(default = "default_quick_preview")]
    pub quick_preview: bool,

    /// Use GitHub avatars for unknown users (default: true)
    /// Set to false to use initials-only avatars
    #[serde(default = "default_github_avatars")]
    pub github_avatars: bool,
}

fn default_quick_preview() -> bool {
    true
}

fn default_github_avatars() -> bool {
    true
}

fn default_title() -> String {
    "Decision Graph".to_string()
}

fn default_primary_color() -> String {
    "#0f3460".to_string()
}

fn default_accent_color() -> String {
    "#e94560".to_string()
}

impl SiteConfig {
    /// Load config from docs/.site.yaml or use defaults (legacy)
    pub fn load(docs_dir: &Path) -> Result<Self> {
        let config = DgConfig::load(docs_dir)?;
        Ok(config.site)
    }

    /// Generate CSS variables from config
    #[allow(dead_code)]
    pub fn to_css_vars(&self) -> String {
        let mut css = String::from(":root {\n");
        css.push_str(&format!("  --primary: {};\n", self.primary_color));
        css.push_str(&format!("  --accent: {};\n", self.accent_color));
        css.push_str("}\n");

        if let Some(ref custom) = self.custom_css {
            css.push_str(custom);
        }

        css
    }
}
