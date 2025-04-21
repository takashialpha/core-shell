use crate::env::get_home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub save_history: bool,
    pub max_history: usize,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct History {
    pub lines: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CoreShellData {
    pub config: Config,
    pub history: History,
}

impl Default for CoreShellData {
    fn default() -> Self {
        Self {
            config: Config::default(),
            history: History { lines: Vec::new() },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            save_history: true,
            max_history: 32,
        }
    }
}

pub struct CoreShellFile;

impl CoreShellFile {
    pub fn load() -> CoreShellData {
        let path = Self::path();
        if !Path::new(&path).exists() {
            let default = CoreShellData::default();
            let _ = fs::write(&path, toml::to_string_pretty(&default).unwrap());
            return default;
        }

        match Self::read_and_parse_file(&path) {
            Ok(parsed) => parsed,
            Err(_) => {
                Self::create_default_file(&path);
                CoreShellData::default()
            }
        }
    }

    pub fn save(data: &CoreShellData) {
        let mut data = data.clone();
        let max = data.config.max_history;
        let len = data.history.lines.len();
        if len > max {
            data.history.lines = data.history.lines[len - max..].to_vec();
        }
        let toml = toml::to_string_pretty(&data).unwrap_or_default();
        let _ = fs::write(Self::path(), toml);
    }

    fn path() -> String {
        match get_home_dir() {
            Some(mut path) => {
                path.push_str("/.coresh");
                path
            }
            None => String::new(),
        }
    }

    fn read_and_parse_file(path: &str) -> Result<CoreShellData, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let parsed = toml::from_str::<CoreShellData>(&content)?;
        Ok(parsed)
    }

    fn create_default_file(path: &str) {
        let default = CoreShellData::default();
        let toml = toml::to_string_pretty(&default).unwrap();
        let _ = fs::write(path, toml);
    }
}
