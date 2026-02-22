// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod config;
mod db;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::logger::log,
            commands::config::get_config,
            commands::database::init_db,
            commands::collection::add_to_collection,
            commands::collection::get_collection,
            commands::collection::remove_from_collection,
            commands::dataset::sync_dataset,
            commands::dataset::get_dataset,
        ])
        .setup(|_| {
            // Initialize database on app startup
            if let Err(e) = db::init(&db::DbLocation::Default) {
                eprintln!("Failed to initialize database: {}", e);
            }
            crate::services::logger::info("[main]", Some("Database initialized successfully"));
            crate::services::logger::info("[main]", Some("Application started successfully"));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
