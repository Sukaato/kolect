#[tauri::command]
pub async fn sync_dataset() -> Result<bool, String> {
    crate::dataset::sync_dataset().await
}
