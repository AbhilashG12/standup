use std::fs;
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::StandupError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,

    #[serde(default)]
    pub repos: Vec<RepoEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub author: Option<String>,

    #[serde(default = "default_since")]
    pub default_since: String,

    pub openai_api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoEntry {
    pub name: String,
    pub path: String,
}

fn default_since() -> String {
    "yesterday".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            author: None,
            default_since: default_since(),
            openai_api_key: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings::default(),
            repos: Vec::new(),
        }
    }
}

pub fn config_path() -> Result<PathBuf, StandupError> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| StandupError::Config("Could not find config directory".to_string()))?;
    Ok(config_dir.join("standup").join("config.toml"))
}

pub fn load() -> Result<Config, StandupError> {
    let path = config_path()?;

    if !path.exists() {
        return Ok(Config::default());
    }

    let contents = fs::read_to_string(&path)
        .map_err(|e| StandupError::Config(format!("Failed to read config: {}", e)))?;

    let config: Config = toml::from_str(&contents)
        .map_err(|e| StandupError::Config(format!("Invalid config file: {}", e)))?;

    Ok(config)
}

pub fn save(config: &Config) -> Result<PathBuf, StandupError> {
    let path = config_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| StandupError::Config(format!("Failed to create config dir: {}", e)))?;
    }

    let contents = toml::to_string_pretty(config)
        .map_err(|e| StandupError::Config(format!("Failed to serialize config: {}", e)))?;

    let mut file = fs::File::create(&path)
        .map_err(|e| StandupError::Config(format!("Failed to write config: {}", e)))?;

    file.write_all(contents.as_bytes())
        .map_err(|e| StandupError::Config(format!("Failed to write config: {}", e)))?;

    Ok(path)
}

pub fn validate(config: &Config) -> Vec<String> {
    config
        .repos
        .iter()
        .filter(|r| !std::path::Path::new(&r.path).exists())
        .map(|r| format!("'{}' at {}", r.name, r.path))
        .collect()
}
