// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod config;
mod entity;
mod schema;
mod services;

use tauri::path::BaseDirectory;
use tauri::Manager;
use tauri_plugin_fs::FsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let scope = app.fs_scope();
            let app_data_dir = app.path().resolve("", BaseDirectory::AppData)?;
            scope.allow_directory(app_data_dir, false)?;

            Ok(())
        })
        .setup(|_app| {
            // Initialize database on app startup
            services::database::init();
            services::logger::info("[setup]", Some("Application started successfully"));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::logger::log,
            commands::config::config_get,
            commands::dataset::dataset_sync,
            commands::dataset::dataset_get,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
