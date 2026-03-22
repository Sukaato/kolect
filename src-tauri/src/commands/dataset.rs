use tauri::{AppHandle, State};
use tauri_plugin_log::log;
use tokio::sync::Mutex;

use crate::commands::collection::CollectionSummaryParams;
use crate::infrastructure::db::repositories::Page;
use crate::services::DatasetService;
use crate::AppStore;

#[tauri::command]
pub async fn dataset_sync(
    app: AppHandle,
    state: State<'_, Mutex<AppStore>>,
    force: bool,
) -> Result<(), String> {
    log::debug!("dataset_sync, force: {}", force);

    let mut state = state.lock().await;
    DatasetService::new(&app, &mut state.db_conn)
        .sync(force)
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn dataset_get_summary(
    app: AppHandle,
    state: State<'_, Mutex<AppStore>>,
    params: CollectionSummaryParams,
) -> Result<serde_json::Value, String> {
    let mut store = state.lock().await;

    let page = Page::new(params.page, params.per_page);

    let mut service = DatasetService::new(&app, &mut store.db_conn);
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
