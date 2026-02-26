use crate::{
    entity::{Album, DatasetDto, Group, Lightstick},
    services::{database::get_db_connection, logger},
};
use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::Manager;

/// Dataset-related services and utilities.
pub async fn sync(app: &tauri::AppHandle) -> Result<bool, String> {
    logger::info("[dataset::sync]", Some("Starting dataset synchronization"));

    // Get dataset_url config
    let dataset_url = crate::config::get_dataset_url();
    let dataset = fetch(&dataset_url).await?;
    let new_version = dataset.dataset_version.clone();
    let current_version = get_dataset_metadata(app).map(|d| d.dataset_version)?;

    // if the version is the same, no need to update
    if new_version == current_version {
        logger::info("[dataset::sync]", Some("Dataset is already up to date"));
        return Ok(false);
    }

    if current_version != "" {
        // Check with semver if the new version is greater than the current version
        let new_semver = semver::Version::parse(&new_version)
            .map_err(|e| format!("Failed to parse new dataset version: {}", e))?;
        let current_semver = semver::Version::parse(&current_version)
            .map_err(|e| format!("Failed to parse current dataset version: {}", e))?;

        if current_semver >= new_semver {
            logger::info(
                "[dataset::sync]",
                Some("New dataset version is not greater than the current version"),
            );
            return Err("New dataset version is not greater than the current version".into());
        }
    }

    // update the data in the database with the new dataset
    update_dataset(dataset.clone())?;

    // Update dataset in database
    update_dataset_metadata(app, &dataset)?;

    logger::info("[dataset::sync]", Some("Dataset synchronization completed"));

    Ok(true)
}

async fn fetch(url: &str) -> Result<DatasetDto, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch dataset: {}", e))?;

    response
        .json::<DatasetDto>()
        .await
        .map_err(|e| format!("Failed to parse dataset: {}", e))
}

fn get_dataset_metadata(app: &tauri::AppHandle) -> Result<DatasetDto, String> {
    let metadata_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let metadata_path = PathBuf::from(metadata_dir).join("dataset_metadata.json");

    if !metadata_path.exists() {
        logger::warn(
            "[dataset::get_dataset_metadata]",
            Some("Dataset metadata file not found"),
        );
        return Ok(DatasetDto::default());
    }

    let metadata_file = fs::read_to_string(&metadata_path)
        .map_err(|e| format!("Failed to read dataset metadata file: {}", e))?;

    match serde_json::from_str::<DatasetDto>(&metadata_file) {
        Ok(dataset) => Ok(dataset),
        Err(e) => {
            logger::warn(
                "[dataset::get_dataset_metadata]",
                Some(&format!("Invalid JSON, returning default dataset: {}", e)),
            );
            Ok(DatasetDto::default())
        }
    }
}

fn update_dataset(dataset: DatasetDto) -> Result<(), String> {
    use crate::schema::{albums, groups, lightsticks};

    let mut connection = get_db_connection();

    // delete all data from groups table
    diesel::delete(groups::table)
        .execute(&mut *connection)
        .map_err(|e| format!("Failed to delete old groups: {}", e))?;

    // bulk insert new data into groups table
    let groups = dataset
        .groups
        .iter()
        .map(|g| g.to_entity())
        .collect::<Vec<_>>();
    diesel::insert_into(groups::table)
        .values(groups)
        .execute(&mut *connection)
        .map_err(|e| format!("Failed to insert new groups: {}", e))?;

    // delete all data from albums table
    diesel::delete(albums::table)
        .execute(&mut *connection)
        .map_err(|e| format!("Failed to delete old albums: {}", e))?;

    // bulk insert new data into albums table
    let albums = dataset
        .albums
        .iter()
        .map(|a| a.to_entity())
        .collect::<Vec<_>>();
    diesel::insert_into(albums::table)
        .values(albums)
        .execute(&mut *connection)
        .map_err(|e| format!("Failed to insert new albums: {}", e))?;

    // delete all data from lightsticks table
    diesel::delete(lightsticks::table)
        .execute(&mut *connection)
        .map_err(|e| format!("Failed to delete old lightsticks: {}", e))?;

    // bulk insert new data into lightsticks table
    let lightsticks = dataset
        .lightsticks
        .iter()
        .map(|l| l.to_entity())
        .collect::<Vec<_>>();
    diesel::insert_into(lightsticks::table)
        .values(lightsticks)
        .execute(&mut *connection)
        .map_err(|e| format!("Failed to insert new lightsticks: {}", e))?;

    Ok(())
}

fn update_dataset_metadata(app: &tauri::AppHandle, dto: &DatasetDto) -> Result<(), String> {
    let metadata_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let metadata_path = PathBuf::from(metadata_dir).join("dataset_metadata.json");

    logger::debug(
        "[dataset::update_dataset_metadata]",
        Some(metadata_path.to_str().unwrap()),
    );

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

pub fn get_dataset(app: &tauri::AppHandle) -> Result<DatasetDto, String> {
    use crate::schema::albums::dsl::albums;
    use crate::schema::groups::dsl::groups;
    use crate::schema::lightsticks::dsl::lightsticks;

    let mut connection = get_db_connection();

    let dataset = get_dataset_metadata(app)?;

    let d_groups = groups
        .select(Group::as_select())
        .load::<Group>(&mut *connection)
        .map_err(|e| format!("Failed to load groups: {}", e))?;

    let d_albums = albums
        .select(Album::as_select())
        .load::<Album>(&mut *connection)
        .map_err(|e| format!("Failed to load albums: {}", e))?;

    let d_lightsticks = lightsticks
        .select(Lightstick::as_select())
        .load::<Lightstick>(&mut *connection)
        .map_err(|e| format!("Failed to load lightsticks: {}", e))?;

    Ok(DatasetDto {
        groups: d_groups.into_iter().map(|g| g.to_dto()).collect(),
        albums: d_albums.into_iter().map(|a| a.to_dto()).collect(),
        lightsticks: d_lightsticks.into_iter().map(|l| l.to_dto()).collect(),
        ..dataset
    })
}
