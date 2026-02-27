use tauri_plugin_log::log;

use crate::config;

/// Get full app configuration (read-only)
#[tauri::command]
pub fn config_get() -> config::AppConfig {
    log::debug!("Retrieving App config");

    config::get_config()
}
