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

        fs::read_to_string(&path)
            .ok()
            .and_then(|c| toml::from_str(&c).ok())
            .unwrap_or_default()
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
}
