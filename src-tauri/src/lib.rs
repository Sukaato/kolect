// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod config;
mod entity;
mod schema;
mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            // Initialize database on app startup
            services::database::init();
            services::logger::info("[setup]", Some("Application started successfully"));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::logger::log,
            commands::config::get_config,
            commands::dataset::dataset_sync,
            commands::dataset::dataset_get,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
