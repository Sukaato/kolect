use tauri::State;
use tokio::sync::Mutex;

use crate::dto::output::{AlbumSummaryDto, FanclubKitItemDto, GroupDetailDto, LightstickItemDto};
use crate::services::GroupService;
use crate::AppStore;

#[tauri::command]
pub async fn group_get_detail(
    state: State<'_, Mutex<AppStore>>,
    group_id: String,
) -> Result<GroupDetailDto, String> {
    let mut store = state.lock().await;

    GroupService::new(&mut store.db_conn)
        .get_detail(&group_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn group_get_album_summaries(
    state: State<'_, Mutex<AppStore>>,
    group_id: String,
    include_exclusive_items: bool,
) -> Result<Vec<AlbumSummaryDto>, String> {
    let mut store = state.lock().await;

    GroupService::new(&mut store.db_conn)
        .get_album_summaries(&group_id, include_exclusive_items)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn group_get_lightsticks(
    state: State<'_, Mutex<AppStore>>,
    group_id: String,
    include_exclusive_items: bool,
) -> Result<Vec<LightstickItemDto>, String> {
    let mut store = state.lock().await;

    GroupService::new(&mut store.db_conn)
        .get_lightsticks(&group_id, include_exclusive_items)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn group_get_fanclub_kits(
    state: State<'_, Mutex<AppStore>>,
    group_id: String,
) -> Result<Vec<FanclubKitItemDto>, String> {
    let mut store = state.lock().await;

    GroupService::new(&mut store.db_conn)
        .get_fanclub_kits(&group_id)
        .map_err(|e| e.to_string())
}
