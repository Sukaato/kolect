use tauri::State;
use tokio::sync::Mutex;

use crate::services::FavoriteService;
use crate::AppStore;

#[tauri::command]
pub async fn favorite_toggle_group(
    state: State<'_, Mutex<AppStore>>,
    group_id: String,
) -> Result<bool, String> {
    let mut store = state.lock().await;
    FavoriteService::new(&mut store.db_conn)
        .toggle_group(&group_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn favorite_toggle_artist(
    state: State<'_, Mutex<AppStore>>,
    artist_id: String,
) -> Result<bool, String> {
    let mut store = state.lock().await;
    FavoriteService::new(&mut store.db_conn)
        .toggle_artist(&artist_id)
        .map_err(|e| e.to_string())
}
