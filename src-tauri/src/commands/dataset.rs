use tauri_plugin_log::log;

use crate::{entity::DatasetDto, services};

/// Syncs the dataset by fetching the latest data from the source and updating the local database accordingly.
#[tauri::command]
pub async fn dataset_sync(app: tauri::AppHandle) -> Result<(), String> {
    log::debug!("sync dataset");
    services::dataset::sync(&app).await?;
    Ok(())
}

/// Retrieves the current dataset from the local database and returns it as a DTO.
#[tauri::command]
pub fn dataset_get(app: tauri::AppHandle) -> Result<DatasetDto, String> {
    log::debug!("get dataset");
    services::dataset::get_dataset(&app)
}
