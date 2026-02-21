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

fn get_dataset_cache_path() -> PathBuf {
    if let Some(data_dir) = dirs::config_dir() {
        data_dir.join("kolect").join("dataset.json")
    } else {
        PathBuf::from("dataset.json")
    }
}

fn get_metadata_cache_path() -> PathBuf {
    if let Some(data_dir) = dirs::config_dir() {
        data_dir.join("kolect").join(".dataset-metadata.json")
    } else {
        PathBuf::from(".dataset-metadata.json")
    }
}

async fn fetch_from_remote(url: &str) -> Result<Dataset, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch dataset: {}", e))?;

    response
        .json::<Dataset>()
        .await
        .map_err(|e| format!("Failed to parse dataset: {}", e))
}

fn load_local_dataset() -> Result<Dataset, String> {
    let path = get_dataset_cache_path();
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read local dataset: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse local dataset: {}", e))
}

fn load_metadata() -> Option<DatasetMetadata> {
    let path = get_metadata_cache_path();
    let content = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

fn save_dataset_cache(dataset: &Dataset) -> Result<(), String> {
    let cache_path = get_dataset_cache_path();
    let metadata_path = get_metadata_cache_path();

    if let Some(parent) = cache_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;
    }

    let dataset_json = serde_json::to_string_pretty(dataset)
        .map_err(|e| format!("Failed to serialize dataset: {}", e))?;

    std::fs::write(&cache_path, dataset_json)
        .map_err(|e| format!("Failed to write dataset: {}", e))?;

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

/// Sync dataset from remote URL
pub async fn sync(url: &str) -> Result<bool, String> {
    let remote_dataset = fetch_from_remote(url).await?;

    let local_dataset = load_local_dataset();
    let metadata = load_metadata();

    let needs_update = match (local_dataset, metadata) {
        (Ok(_local), Some(meta)) => remote_dataset.dataset_version != meta.version,
        _ => true,
    };

    if needs_update {
        save_dataset_cache(&remote_dataset)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
