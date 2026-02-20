// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod dataset;
mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::database::init_db,
            commands::collection::add_to_collection,
            commands::collection::get_collection,
            commands::collection::remove_from_collection,
            commands::dataset::sync_dataset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
