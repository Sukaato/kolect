use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub log: LogConfig,
    pub dataset: DatasetConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }

    pub fn should_log(&self, level: &LogLevel) -> bool {
        level >= self
    }
}

fn load_config() -> AppConfig {
    let content =
        std::fs::read_to_string("config.toml").expect("[config] Failed to read config.toml");

    toml::from_str::<AppConfig>(&content).expect("[config] Failed to parse config.toml")
}

lazy_static! {
    static ref CONFIG: Mutex<AppConfig> = Mutex::new(load_config());
}

pub fn get_config() -> AppConfig {
    CONFIG.lock().unwrap().clone()
}

pub fn get_log_level() -> LogLevel {
    let config = CONFIG.lock().unwrap();
    LogLevel::from_str(&config.log.level)
}

pub fn get_dataset_url() -> String {
    CONFIG.lock().unwrap().dataset.url.clone()
}
