use tauri::{AppHandle, State};
use tauri_plugin_log::log;
use tokio::sync::Mutex;

use crate::services::dataset::DatasetService;
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
