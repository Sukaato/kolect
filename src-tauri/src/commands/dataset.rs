use tauri_plugin_log::log;

use crate::entity::DatasetDto;
use crate::services::dataset as dataset_service;

/// Tauri command — synchronizes the local dataset against the remote source.
///
/// Fetches the latest dataset from the configured URL and conditionally updates
/// the local SQLite database and metadata file based on version comparison.
///
/// # Sync behavior
///
/// | Condition                                | Action         | Returns       |
/// |------------------------------------------|----------------|---------------|
/// | `force = true`                           | Always updates | `Ok(true)`    |
/// | Versions are equal                       | Skips update   | `Ok(false)`   |
/// | Current version is empty (first launch)  | Updates        | `Ok(true)`    |
/// | Remote version > local version           | Updates        | `Ok(true)`    |
/// | Remote version ≤ local version           | Rejects        | `Err(...)`    |
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `force` - Bypasses version checks and forces a full update.
///
/// # Returns
/// * `Ok(true)` - The dataset was updated.
/// * `Ok(false)` - The dataset was already up to date, no update needed.
/// * `Err(String)` - A human-readable error if any step fails.
///
/// # Errors
/// * Fetch failure → see [`dataset_service::fetch`]
/// * Metadata read failure → see [`dataset_service::get_dataset_metadata`]
/// * Invalid semver string → `"Failed to parse new/current dataset version: ..."`
/// * Remote version not greater → `"New dataset version is not greater than the current version"`
/// * Database update failure → see [`dataset_service::update_dataset`]
/// * Metadata write failure → see [`dataset_service::update_dataset_metadata`]
#[tauri::command]
pub async fn dataset_sync(app: tauri::AppHandle, force: bool) -> Result<bool, String> {
    log::debug!("dataset_sync, force: {}", force);

    // Get dataset_url config
    let dataset_url = crate::config::get_dataset_url();
    let dataset = dataset_service::fetch(&dataset_url).await?;
    let new_version = dataset.dataset_version.clone();
    let current_version = dataset_service::get_dataset_metadata(&app).map(|d| d.dataset_version)?;

    if force {
        // update the data in the database with the new dataset
        dataset_service::update_dataset(dataset.clone())?;

        // Update dataset in database
        dataset_service::update_dataset_metadata(&app, &dataset)?;

        log::info!("Dataset synchronization completed");

        return Ok(true);
    }

    // if the version is the same, no need to update
    if new_version == current_version {
        log::info!("Dataset is already up to date");
        return Ok(false);
    }

    if !current_version.is_empty() {
        // Check with semver if the new version is greater than the current version
        let new_semver = semver::Version::parse(&new_version)
            .map_err(|e| format!("Failed to parse new dataset version: {}", e))?;
        let current_semver = semver::Version::parse(&current_version)
            .map_err(|e| format!("Failed to parse current dataset version: {}", e))?;

        if current_semver >= new_semver {
            log::warn!("New dataset version is not greater than the current version");
            return Err("New dataset version is not greater than the current version".into());
        }
    }

    // update the data in the database with the new dataset
    dataset_service::update_dataset(dataset.clone())?;

    // Update dataset in database
    dataset_service::update_dataset_metadata(&app, &dataset)?;

    log::info!("Dataset synchronization completed");

    Ok(true)
}

/// Tauri command — retrieves the current dataset from local storage.
///
/// Thin wrapper around [`dataset_service::get_dataset`].
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// * `Ok(DatasetDto)` - The assembled dataset.
/// * `Err(String)` - A human-readable error if any source fails to load.
///
/// # Errors
/// See [`dataset_service::get_dataset`].
#[tauri::command]
pub fn dataset_get(app: tauri::AppHandle) -> Result<DatasetDto, String> {
    log::debug!("dataset_get");

    dataset_service::get_dataset(&app)
}
