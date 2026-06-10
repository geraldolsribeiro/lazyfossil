use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    pub editor: Option<String>,
    pub binary_glob: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        fs::read_to_string(path)
            .ok()
            .and_then(|content| toml::from_str::<Config>(&content).ok())
            .unwrap_or_default()
    }
}

fn config_path() -> PathBuf {
    if let Ok(path) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(path).join("lazyfossil").join("config.toml");
    }

    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home)
            .join(".config")
            .join("lazyfossil")
            .join("config.toml");
    }

    PathBuf::from("config.toml")
}
