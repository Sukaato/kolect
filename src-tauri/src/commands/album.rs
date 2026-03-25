use tauri::State;
use tokio::sync::Mutex;

use crate::dto::output::{AlbumDetailDto, AlbumVersionItemDto, DigipackItemDto, PhotocardItemDto};
use crate::services::AlbumService;
use crate::AppStore;

#[tauri::command]
pub async fn album_get_detail(
    state: State<'_, Mutex<AppStore>>,
    album_id: String,
    include_exclusive_items: bool,
) -> Result<AlbumDetailDto, String> {
    let mut store = state.lock().await;

    AlbumService::new(&mut store.db_conn)
        .get_detail(&album_id, include_exclusive_items)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn album_get_versions(
    state: State<'_, Mutex<AppStore>>,
    album_id: String,
    include_exclusive_items: bool,
) -> Result<Vec<AlbumVersionItemDto>, String> {
    let mut store = state.lock().await;

    AlbumService::new(&mut store.db_conn)
        .get_versions(&album_id, include_exclusive_items)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn album_get_digipacks(
    state: State<'_, Mutex<AppStore>>,
    album_id: String,
    include_exclusive_items: bool,
) -> Result<Vec<DigipackItemDto>, String> {
    let mut store = state.lock().await;

    AlbumService::new(&mut store.db_conn)
        .get_digipacks(&album_id, include_exclusive_items)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn album_get_photocards(
    state: State<'_, Mutex<AppStore>>,
    album_id: String,
    include_exclusive_items: bool,
) -> Result<Vec<PhotocardItemDto>, String> {
    let mut store = state.lock().await;

    AlbumService::new(&mut store.db_conn)
        .get_photocards(&album_id, include_exclusive_items)
        .map_err(|e| e.to_string())
}
