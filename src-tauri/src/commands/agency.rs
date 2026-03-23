use tauri::State;
use tokio::sync::Mutex;

use crate::dto::output::AgencyDto;
use crate::services::AgencyService;
use crate::AppStore;

/// Returns all non-deleted agencies — used to populate the agency filter on the Home screen.
#[tauri::command]
pub async fn dataset_get_agencies(
    state: State<'_, Mutex<AppStore>>,
) -> Result<Vec<AgencyDto>, String> {
    let mut store = state.lock().await;
    let mut service = AgencyService::new(&mut store.db_conn);

    service.get_all().map_err(|e| e.to_string())
}

/// Returns only agencies with at least one item in the user collection
/// — used to populate the agency filter on the Collection screen.
#[tauri::command]
pub async fn collection_get_agencies(
    state: State<'_, Mutex<AppStore>>,
) -> Result<Vec<AgencyDto>, String> {
    let mut store = state.lock().await;
    let mut service = AgencyService::new(&mut store.db_conn);

    service.get_for_collection().map_err(|e| e.to_string())
}
