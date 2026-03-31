use tauri_plugin_fs::FsExt;

use serde::Deserialize;
use tauri::{path::BaseDirectory, App, Manager};
use tauri_plugin_log::log::info;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub dataset_url: String,
}

impl AppConfig {
    pub fn load(app: &mut App) -> AppConfig {
        let filename = if cfg!(dev) {
            "config/dev.json"
        } else {
            "config/prod.json"
        };

        let path = app
            .path()
            .resolve(filename, BaseDirectory::Resource)
            .expect("failed to resolve config path");
        info!("config path: {:#?}", path);

        let content = app
            .fs()
            .read_to_string(path)
            .expect("failed to read config content");

        serde_json::from_str(&content).expect("failed to parse config content")
    }
}
