use tauri::State;
use tokio::sync::Mutex;

use crate::db::repositories::{Page, PaginatedResult};
use crate::dto::input::commands::{CollectionSummaryParams, UpdateItemDto};
use crate::dto::output::CollectionSummaryItem;
use crate::services::{CollectionItemService, CollectionService};
use crate::AppStore;

#[tauri::command]
pub async fn collection_get_summary(
    state: State<'_, Mutex<AppStore>>,
    params: CollectionSummaryParams,
) -> Result<PaginatedResult<CollectionSummaryItem>, String> {
    let mut store = state.lock().await;

    let page = Page::new(params.page, params.per_page);

    let mut service = CollectionService::new(&mut store.db_conn);

    service
        .get_summary(
            page,
            params.search.as_deref(),
            params.agency_id.as_deref(),
            params.include_photocards,
            params.include_exclusive_items,
        )
        .map_err(|e| e.to_string())
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
