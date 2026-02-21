use crate::services::logger;

/// Sync dataset from remote source
#[tauri::command]
pub async fn sync_dataset() -> Result<bool, String> {
    logger::info("[sync_dataset] Starting dataset sync", None);

    let url = crate::config::get_dataset_url();
    let result = crate::services::dataset::sync(&url).await;

    match &result {
        Ok(updated) => {
            logger::info(
                "[sync_dataset] Dataset sync completed",
                Some(&format!("updated={}", updated)),
            );
        }
        Err(e) => logger::error("[sync_dataset] Dataset sync failed", Some(e)),
    }

    result
}
