use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub log: LogConfigResponse,
    pub dataset: DatasetConfigResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfigResponse {
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfigResponse {
    pub url: String,
}

/// Get full app configuration (read-only)
#[tauri::command]
pub fn get_config() -> ConfigResponse {
    println!("[get_config] Retrieving app configuration");

    let config = crate::config::get_config();
    ConfigResponse {
        log: LogConfigResponse {
            level: config.log.level,
        },
        dataset: DatasetConfigResponse {
            url: config.dataset.url,
        },
    }
}
