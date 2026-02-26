use crate::{entity::DatasetDto, services};

/// Syncs the dataset by fetching the latest data from the source and updating the local database accordingly.
#[tauri::command]
pub async fn dataset_sync() -> Result<(), String> {
    services::dataset::sync().await?;
    Ok(())
}

/// Retrieves the current dataset from the local database and returns it as a DTO.
#[tauri::command]
pub fn dataset_get() -> Result<DatasetDto, String> {
    services::dataset::get_dataset()
}
