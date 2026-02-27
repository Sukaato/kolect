// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod config;
mod entity;
mod schema;
mod services;

use chrono::{SecondsFormat, Utc};
use tauri::path::BaseDirectory;
use tauri::Manager;
use tauri_plugin_fs::FsExt;
use tauri_plugin_log::log;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Debug)
                .format(|out, message, record| {
                    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, false);

                    out.finish(format_args!(
                        "{} [{}] [{}] - {}",
                        now,
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .build(),
        )
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
            log::info!("Application started successfully");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::config::config_get,
            commands::dataset::dataset_sync,
            commands::dataset::dataset_get,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
