use tauri::State;
use tokio::sync::Mutex;

use crate::dto::output::{AlbumSummaryDto, ArtistDetailDto, FanclubKitItemDto, LightstickItemDto};
use crate::services::ArtistService;
use crate::AppStore;

#[tauri::command]
pub async fn artist_get_detail(
    state: State<'_, Mutex<AppStore>>,
    artist_id: String,
) -> Result<ArtistDetailDto, String> {
    let mut store = state.lock().await;

    ArtistService::new(&mut store.db_conn)
        .get_detail(&artist_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn artist_get_album_summaries(
    state: State<'_, Mutex<AppStore>>,
    artist_id: String,
    include_exclusive_items: bool,
) -> Result<Vec<AlbumSummaryDto>, String> {
    let mut store = state.lock().await;

    ArtistService::new(&mut store.db_conn)
        .get_album_summaries(&artist_id, include_exclusive_items)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn artist_get_lightsticks(
    state: State<'_, Mutex<AppStore>>,
    artist_id: String,
    include_exclusive_items: bool,
) -> Result<Vec<LightstickItemDto>, String> {
    let mut store = state.lock().await;

    ArtistService::new(&mut store.db_conn)
        .get_lightsticks(&artist_id, include_exclusive_items)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn artist_get_fanclub_kits(
    state: State<'_, Mutex<AppStore>>,
    artist_id: String,
) -> Result<Vec<FanclubKitItemDto>, String> {
    let mut store = state.lock().await;

    ArtistService::new(&mut store.db_conn)
        .get_fanclub_kits(&artist_id)
        .map_err(|e| e.to_string())
}
