use crate::services::dataset::{ Dataset, sync, get };
use crate::services::logger;

/// Sync dataset from remote source
#[tauri::command]
pub async fn sync_dataset() -> Result<bool, String> {
    logger::info("[sync_dataset] Starting dataset sync", None);

    let url = crate::config::get_dataset_url();
    let result = sync(&url).await;

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

/// get dataset
#[tauri::command]
pub fn get_dataset() -> Result<Dataset, String> {
    logger::info("[get_dataset]", Some("Fetching dataset..."));

    let result = get();

    match &result {
        Ok(dataset) => logger::info(
            "[get_dataset] Dataset fetched successfully",
            Some(&format!(
                "groups={}, albums={}, lightsticks={}",
                dataset.groups.len(),
                dataset.albums.len(),
                dataset.lightsticks.len()
            )),
        ),
        Err(e) => logger::error("[get_dataset] Failed to fetch dataset", Some(e)),
    }

    result
}
