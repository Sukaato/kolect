use tauri::{AppHandle, State};
use tauri_plugin_log::log;
use tokio::sync::Mutex;

use crate::db::repositories::{Page, PaginatedResult};
use crate::dto::input::commands::CollectionSummaryParams;
use crate::dto::output::CollectionSummaryItem;
use crate::services::DatasetService;
use crate::AppStore;

#[tauri::command]
pub async fn dataset_sync(
    app: AppHandle,
    state: State<'_, Mutex<AppStore>>,
    force: bool,
) -> Result<(), String> {
    log::debug!("dataset_sync, force: {}", force);

    let mut store = state.lock().await;
    let mut service = DatasetService::new(&app, &mut store);
    service.sync(force).await?;

    Ok(())
}

#[tauri::command]
pub async fn dataset_get_summary(
    app: AppHandle,
    state: State<'_, Mutex<AppStore>>,
    params: CollectionSummaryParams,
) -> Result<PaginatedResult<CollectionSummaryItem>, String> {
    let mut store = state.lock().await;

    let page = Page::new(params.page, params.per_page);

    let mut service = DatasetService::new(&app, &mut store);

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
