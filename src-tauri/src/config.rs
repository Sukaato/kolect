use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub dataset: DatasetConfig,
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

pub fn get_dataset_url() -> String {
    CONFIG.lock().unwrap().dataset.url.clone()
}
