// src-tauri/src/lib.rs

mod commands;
mod dto;
mod infrastructure;
mod services;

use diesel::SqliteConnection;
use tauri::Manager;
use tauri_plugin_log::log;
use tokio::sync::Mutex;

struct AppStore {
    db_conn: SqliteConnection,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if cfg!(dev) {
        dotenvy::dotenv().ok();
    } else {
        dotenvy::from_filename(".env.production").ok();
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(build_logger().build())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            // dataset
            commands::dataset::dataset_sync,
            commands::dataset::dataset_get_summary,
            // collection
            commands::collection::collection_get_summary,
            commands::collection::collection_update_item,
            // group
            commands::group::group_get_detail,
            commands::group::group_get_album_summaries,
            commands::group::group_get_lightsticks,
            commands::group::group_get_fanclub_kits,
            // album
            commands::album::album_get_detail,
            commands::album::album_get_versions,
            commands::album::album_get_digipacks,
            commands::album::album_get_photocards,
            // artist
            commands::artist::artist_get_detail,
            commands::artist::artist_get_album_summaries,
            commands::artist::artist_get_lightsticks,
            commands::artist::artist_get_fanclub_kits,
            // favorite
            commands::favorite::favorite_toggle_group,
            commands::favorite::favorite_toggle_artist,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ─── Setup ────────────────────────────────────────────────────────────────────

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::path::BaseDirectory;
    use tauri_plugin_fs::FsExt;

    let scope = app.fs_scope();
    let app_data_dir = app.path().resolve("", BaseDirectory::AppData)?;
    scope.allow_directory(app_data_dir, false)?;
    log::info!("Fs scope setup successfully");

    services::database_service::init(app.handle());
    let mut conn = services::database_service::establish_db_connection(app.handle());
    services::database_service::migrate(&mut conn);
    log::info!("Local database setup successfully");

    app.manage(Mutex::new(AppStore { db_conn: conn }));
    log::info!("App managed state created");

    log::info!("Application started successfully");
    Ok(())
}

// ─── Logger ───────────────────────────────────────────────────────────────────

fn build_logger() -> tauri_plugin_log::Builder {
    use chrono::{SecondsFormat, Utc};

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
}
