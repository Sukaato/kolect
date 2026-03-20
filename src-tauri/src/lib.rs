// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod dto;
mod infrastructure;
mod services;

use chrono::{SecondsFormat, Utc};
use diesel::SqliteConnection;
use tokio::sync::Mutex;

use tauri::path::BaseDirectory;
use tauri::Manager;
use tauri_plugin_fs::FsExt;
use tauri_plugin_log::log;

struct AppStore {
    db_conn: SqliteConnection,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if cfg!(dev) {
        dotenvy::dotenv().ok(); // charge .env en dev
    } else {
        dotenvy::from_filename(".env.production").ok();
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Debug)
                .format(|out, message, record| {
                    let now = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, false);

                    let mut target = record.target();
                    if target.starts_with("webview") {
                        target = "kolect_front";
                    }

                    out.finish(format_args!(
                        "{} [{}] [{}] - {}",
                        now,
                        record.level(),
                        target,
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

            log::info!("Fs scope setup successfully");

            services::database_service::init();
            let mut conn = services::database_service::establish_db_connection();
            services::database_service::migrate(&mut conn);
            log::info!("Local database setup successfully");

            app.manage(Mutex::new(AppStore { db_conn: conn }));
            log::info!("App managed state created");

            log::info!("Application started successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::dataset::dataset_sync,
            commands::dataset::dataset_get_summary,
            commands::collection::collection_get_summary,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
