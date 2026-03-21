use tauri::State;
use tokio::sync::Mutex;

use crate::services::GroupService;
use crate::AppStore;

#[tauri::command]
pub async fn group_get_detail(
    state: State<'_, Mutex<AppStore>>,
    group_id: String,
) -> Result<serde_json::Value, String> {
    let mut store = state.lock().await;
    let result = GroupService::new(&mut store.db_conn)
        .get_detail(&group_id)
        .map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn group_get_album_summaries(
    state: State<'_, Mutex<AppStore>>,
    group_id: String,
) -> Result<serde_json::Value, String> {
    let mut store = state.lock().await;
    let result = GroupService::new(&mut store.db_conn)
        .get_album_summaries(&group_id)
        .map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn group_get_lightsticks(
    state: State<'_, Mutex<AppStore>>,
    group_id: String,
) -> Result<serde_json::Value, String> {
    let mut store = state.lock().await;
    let result = GroupService::new(&mut store.db_conn)
        .get_lightsticks(&group_id)
        .map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn group_get_fanclub_kits(
    state: State<'_, Mutex<AppStore>>,
    group_id: String,
) -> Result<serde_json::Value, String> {
    let mut store = state.lock().await;
    let result = GroupService::new(&mut store.db_conn)
        .get_fanclub_kits(&group_id)
        .map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}
