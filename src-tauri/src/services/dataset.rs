use crate::{
    entity::{Collectible, CollectibleDto, DatasetDto, Group, GroupDto},
    services::database::get_db_connection,
};
use diesel::RunQueryDsl;
use std::{
    fs::{self, File},
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::Manager;
use tauri_plugin_log::log;

/// Fetches and deserializes a dataset from a remote URL.
///
/// # Arguments
/// * `url` - The endpoint to fetch the dataset from.
///
/// # Returns
/// * `Ok(DatasetDto)` - The deserialized dataset.
/// * `Err(String)` - A human-readable error if the request or deserialization fails.
///
/// # Errors
/// * Network error → `"Failed to fetch dataset: ..."`
/// * Invalid/unexpected JSON → `"Failed to parse dataset: ..."`
pub async fn fetch(url: &str) -> Result<DatasetDto, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch dataset: {}", e))?;

    response
        .json::<DatasetDto>()
        .await
        .map_err(|e| format!("Failed to parse dataset: {}", e))
}

/// Reads and deserializes the dataset metadata from the app's local data directory.
///
/// Falls back to [`DatasetDto::default`] instead of propagating an error in two cases:
/// * The metadata file does not exist yet (first launch).
/// * The file contains invalid JSON (corrupted file).
///
/// # Arguments
/// * `app` - The Tauri application handle, used to resolve the data directory.
///
/// # Returns
/// * `Ok(DatasetDto)` - The deserialized dataset, or a default value as fallback.
/// * `Err(String)` - Only if the data directory cannot be resolved, or the file cannot be read.
///
/// # Errors
/// * Data directory unresolvable → from [`tauri::path::PathResolver`]
/// * File read failure → `"Failed to read dataset metadata file: ..."`
pub fn get_dataset_metadata(app: &tauri::AppHandle) -> Result<DatasetDto, String> {
    let metadata_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let metadata_path = metadata_dir.join("dataset_metadata.json");

    if !metadata_path.exists() {
        log::warn!("Dataset metadata file not found");
        return Ok(DatasetDto::default());
    }

    let metadata_file = fs::read_to_string(&metadata_path)
        .map_err(|e| format!("Failed to read dataset metadata file: {}", e))?;

    match serde_json::from_str::<DatasetDto>(&metadata_file) {
        Ok(dataset) => Ok(dataset),
        Err(e) => {
            log::error!("Invalid JSON, returning default dataset: {}", e);
            Ok(DatasetDto::default())
        }
    }
}

/// Replaces the entire dataset in the local SQLite database using a delete-then-insert strategy.
///
/// Each table is fully cleared before the new data is inserted.
/// This operation is **not atomic** — if a step fails midway, previously deleted data
/// will not be restored. Consider wrapping in a transaction if integrity is required.
///
/// # Arguments
/// * `dataset` - The new dataset to persist, consumed by this function.
///
/// # Returns
/// * `Ok(())` - All tables were successfully replaced.
/// * `Err(String)` - A human-readable error if any delete or insert step fails.
///
/// # Errors
/// * Groups delete failure → `"Failed to delete old groups: ..."`
/// * Groups insert failure → `"Failed to insert new groups: ..."`
/// * Collectibles delete failure → `"Failed to delete old collectibles: ..."`
/// * Collectibles insert failure → `"Failed to insert collectibles: ..."`
pub fn update_dataset(dataset: DatasetDto) -> Result<(), String> {
    use crate::schema::{collectibles, groups};

    let mut connection = get_db_connection();

    // delete all data from groups table
    diesel::delete(groups::table)
        .execute(&mut *connection)
        .map_err(|e| format!("Failed to delete old groups: {}", e))?;

    // bulk insert new data into groups table
    let groups = dataset
        .groups
        .into_iter()
        .map(Group::from)
        .collect::<Vec<_>>();
    diesel::insert_into(groups::table)
        .values(groups)
        .execute(&mut *connection)
        .map_err(|e| format!("Failed to insert new groups: {}", e))?;

    // delete all collectibles
    diesel::delete(collectibles::table)
        .execute(&mut *connection)
        .map_err(|e| format!("Failed to delete old collectibles: {}", e))?;

    // bulk insert new data into albums table
    let collectibles = dataset
        .collectibles
        .into_iter()
        .map(Collectible::from)
        .collect::<Vec<_>>();
    diesel::insert_into(collectibles::table)
        .values(collectibles)
        .execute(&mut *connection)
        .map_err(|e| format!("Failed to insert collectibles: {}", e))?;

    Ok(())
}

/// Writes dataset metadata to a JSON file in the app's local data directory.
///
/// Only a subset of [`DatasetDto`] is persisted: the dataset version, its generation date,
/// and a `lastFetchedAt` timestamp generated at call time from the system clock.
///
/// The file is created if it does not exist, or fully overwritten if it does.
///
/// # Arguments
/// * `app` - The Tauri application handle, used to resolve the data directory.
/// * `dto` - The dataset DTO to extract metadata from.
///
/// # Returns
/// * `Ok(())` - The metadata file was successfully written.
/// * `Err(String)` - A human-readable error if any step of the pipeline fails.
///
/// # Errors
/// * Data directory unresolvable → from [`tauri::path::PathResolver`]
/// * System clock before Unix epoch → `"Time error: ..."`
/// * Serialization failure → `"Can not pretty string: ..."`
/// * File creation failure → `"Can not create dataset_metadata file: ..."`
/// * Write failure → `"Can not write in dataset_metadata file: ..."`
pub fn update_dataset_metadata(app: &tauri::AppHandle, dto: &DatasetDto) -> Result<(), String> {
    let metadata_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let metadata_path = metadata_dir.join("dataset_metadata.json");

    log::debug!("dataset_metadata path: {}", metadata_path.to_str().unwrap());

    // Current timestamp (no external deps)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_secs();

    // JSON content
    let metadata = serde_json::json!({
        "datasetVersion": dto.dataset_version,
        "generatedAt": dto.generated_at,
        "lastFetchedAt": now
    });
    let content = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("Can not pretty string: {}", e))?;

    // Write to file

    // std::fs::write(path, content)
    let mut file = File::create(&metadata_path)
        .map_err(|e| format!("Can not create dataset_metadata file: {}", e))?;
    file.write_all(content.as_bytes())
        .map_err(|e| format!("Can not write in dataset_metadata file: {}", e))?;

    Ok(())
}

/// Assembles a full [`DatasetDto`] from the local SQLite database and metadata file.
///
/// Metadata fields (`dataset_version`, `generated_at`, `last_fetched_at`) are sourced
/// from the metadata file via [`get_dataset_metadata`], while `groups` and `collectibles`
/// are loaded from the database and override the DTO's corresponding fields.
///
/// # Arguments
/// * `app` - The Tauri application handle, used to resolve the metadata file path.
///
/// # Returns
/// * `Ok(DatasetDto)` - The fully assembled dataset.
/// * `Err(String)` - A human-readable error if any source fails to load.
///
/// # Errors
/// * Metadata read failure → see [`get_dataset_metadata`]
/// * Groups load failure → `"Failed to load groups: ..."`
/// * Collectibles load failure → `"Failed to load collectibles: ..."`
pub fn get_dataset(app: &tauri::AppHandle) -> Result<DatasetDto, String> {
    use crate::schema::collectibles::dsl::collectibles;
    use crate::schema::groups::dsl::groups;

    let mut connection = get_db_connection();

    let dataset = get_dataset_metadata(app)?;

    let d_groups = groups
        .load::<Group>(&mut *connection)
        .map_err(|e| format!("Failed to load groups: {}", e))?;

    let d_collectibles = collectibles
        .load::<Collectible>(&mut *connection)
        .map_err(|e| format!("Failed to load collectibles: {}", e))?;

    Ok(DatasetDto {
        groups: d_groups.into_iter().map(GroupDto::from).collect(),
        collectibles: d_collectibles
            .into_iter()
            .map(CollectibleDto::from)
            .collect(),
        ..dataset
    })
}
