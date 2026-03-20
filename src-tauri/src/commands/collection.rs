// src-tauri/src/commands/collection.rs

use tauri::State;
use tokio::sync::Mutex;

use crate::infrastructure::db::repositories::Page;
use crate::services::collection_service::{CollectionService, CollectionSortBy};
use crate::AppStore;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSummaryParams {
    pub page: u32,
    pub per_page: u32,
    pub sort_by: String, // "name" | "agency"
    pub include_photocards: bool,
}

#[tauri::command]
pub async fn collection_get_summary(
    state: State<'_, Mutex<AppStore>>,
    params: CollectionSummaryParams,
) -> Result<serde_json::Value, String> {
    let mut store = state.lock().await;

    let sort_by = match params.sort_by.as_str() {
        "agency" => CollectionSortBy::Agency,
        _ => CollectionSortBy::Name,
    };

    let page = Page::new(params.page, params.per_page);

    let mut service = CollectionService::new(&mut store.db_conn);
    let result = service
        .get_summary(page, sort_by, params.include_photocards)
        .map_err(|e| e.to_string())?;

    serde_json::to_value(result).map_err(|e| e.to_string())
}
