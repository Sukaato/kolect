use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetadata {
    pub version: String,
    pub fetched_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(rename = "datasetVersion")]
    pub dataset_version: String,
    #[serde(rename = "generatedAt")]
    pub generated_at: String,
}

/// Get the local dataset cache path
fn get_dataset_cache_path() -> PathBuf {
    if let Some(data_dir) = dirs::config_dir() {
        data_dir.join("kolect").join("dataset.json")
    } else {
        PathBuf::from("dataset.json")
    }
}

/// Get the local metadata cache path
fn get_metadata_cache_path() -> PathBuf {
    if let Some(data_dir) = dirs::config_dir() {
        data_dir.join("kolect").join(".dataset-metadata.json")
    } else {
        PathBuf::from(".dataset-metadata.json")
    }
}

/// Fetch dataset from GitHub
async fn fetch_from_github() -> Result<Dataset, String> {
    let url = "https://raw.githubusercontent.com/Sukaato/kolect/main/public/dataset.json";

    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch dataset: {}", e))?;

    response
        .json::<Dataset>()
        .await
        .map_err(|e| format!("Failed to parse dataset: {}", e))
}

/// Load local dataset from cache
fn load_local_dataset() -> Result<Dataset, String> {
    let path = get_dataset_cache_path();

    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read local dataset: {}", e))?;

    serde_json::from_str(&content).map_err(|e| format!("Failed to parse local dataset: {}", e))
}

/// Load local metadata
fn load_metadata() -> Option<DatasetMetadata> {
    let path = get_metadata_cache_path();

    let content = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Save dataset and metadata to cache
fn save_dataset_cache(dataset: &Dataset) -> Result<(), String> {
    let cache_path = get_dataset_cache_path();
    let metadata_path = get_metadata_cache_path();

    // Ensure directory exists
    if let Some(parent) = cache_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;
    }

    // Save dataset
    let dataset_json = serde_json::to_string_pretty(dataset)
        .map_err(|e| format!("Failed to serialize dataset: {}", e))?;

    std::fs::write(&cache_path, dataset_json)
        .map_err(|e| format!("Failed to write dataset: {}", e))?;

    // Save metadata
    let metadata = DatasetMetadata {
        version: dataset.dataset_version.clone(),
        fetched_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    };

    let metadata_json = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;

    std::fs::write(&metadata_path, metadata_json)
        .map_err(|e| format!("Failed to write metadata: {}", e))?;

    Ok(())
}

/// Sync dataset from GitHub, comparing versions and saving if different
pub async fn sync_dataset() -> Result<bool, String> {
    // Fetch from GitHub
    let remote_dataset = match fetch_from_github().await {
        Ok(dataset) => dataset,
        Err(e) => {
            eprintln!("Failed to fetch from GitHub: {}", e);
            // Return success but indicate no update
            return Ok(false);
        }
    };

    // Try to load local dataset
    let local_dataset = load_local_dataset();
    let metadata = load_metadata();

    // Check if update is needed
    let needs_update = match (local_dataset, metadata) {
        (Ok(_local), Some(meta)) => {
            // Compare versions
            remote_dataset.dataset_version != meta.version
        }
        _ => {
            // No local cache, always update
            true
        }
    };

    if needs_update {
        save_dataset_cache(&remote_dataset)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Get dataset, preferring local cache, falling back to remote
#[allow(dead_code)]
pub async fn get_dataset() -> Result<Dataset, String> {
    // Try local first
    if let Ok(local) = load_local_dataset() {
        return Ok(local);
    }

    // Fall back to remote
    fetch_from_github().await
}
