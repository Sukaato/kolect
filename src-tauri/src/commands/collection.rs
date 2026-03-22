use tauri::State;
use tokio::sync::Mutex;

use crate::dto::input::UpdateItemDto;
use crate::infrastructure::db::repositories::Page;
use crate::services::{CollectionItemService, CollectionService};
use crate::AppStore;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSummaryParams {
    pub page: u32,
    pub per_page: u32,
    pub include_photocards: bool,
    pub search: Option<String>,
    pub agency_id: Option<String>,
}

#[tauri::command]
pub async fn collection_get_summary(
    state: State<'_, Mutex<AppStore>>,
    params: CollectionSummaryParams,
) -> Result<serde_json::Value, String> {
    let mut store = state.lock().await;

    let page = Page::new(params.page, params.per_page);

    let mut service = CollectionService::new(&mut store.db_conn);
    let result = service
        .get_summary(
            page,
            params.search.as_deref(),
            params.agency_id.as_deref(),
            params.include_photocards,
        )
        .map_err(|e| e.to_string())?;

    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn collection_update_item(
    state: State<'_, Mutex<AppStore>>,
    item_type: String,
    item_id: String,
    owned_count: u32,
    signed_count: u32,
) -> Result<(), String> {
    let item_type = serde_json::from_value(serde_json::Value::String(item_type))
        .map_err(|e| format!("Type invalide : {}", e))?;

    let dto = UpdateItemDto {
        item_type,
        item_id,
        owned_count,
        signed_count,
    };

    let mut store = state.lock().await;
    CollectionItemService::new(&mut store.db_conn)
        .update_item(dto)
        .map_err(|e| e.to_string())
}
