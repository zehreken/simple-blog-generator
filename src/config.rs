use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SbgConfig {
    pub site: SiteConfig,
    pub analytics: Option<AnalyticsConfig>,
    pub nav: Option<NavConfig>,
    pub footer: Option<FooterConfig>,
    pub paths: Option<PathsConfig>,
}

impl Default for SbgConfig {
    fn default() -> Self {
        SbgConfig {
            site: SiteConfig::default(),
            analytics: None,
            nav: None,
            footer: None,
            paths: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SiteConfig {
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub language: Option<String>,
}

impl Default for SiteConfig {
    fn default() -> Self {
        SiteConfig {
            name: "My Blog".to_string(),
            description: Some("A personal blog".to_string()),
            url: None,
            language: Some("en_US".to_string()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AnalyticsConfig {
    pub google_tag: Option<String>,
    pub librecounter: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NavLink {
    pub label: String,
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct NavConfig {
    pub links: Vec<NavLink>,
}

#[derive(Debug, Deserialize)]
pub struct FooterConfig {
    pub links: Vec<NavLink>,
}

#[derive(Debug, Deserialize)]
pub struct PathsConfig {
    pub posts: Option<String>,
    pub zettelkasten: Option<String>,
    pub output: Option<String>,
}

pub fn load(project_root: &Path) -> Result<SbgConfig, Box<dyn std::error::Error>> {
    let config_path = project_root.join("sbg.toml");
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        Ok(toml::from_str(&content)?)
    } else {
        Err("No sbg.toml found in current directory. Run `sbg init` to create one.".into())
    }
}
