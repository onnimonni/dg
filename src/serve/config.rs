use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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
    /// Load config from docs/.site.yaml or use defaults
    pub fn load(docs_dir: &Path) -> Result<Self> {
        let config_path = docs_dir.join(".site.yaml");

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: SiteConfig = serde_yaml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
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
