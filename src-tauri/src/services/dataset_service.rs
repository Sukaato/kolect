use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use semver::Version;
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_log::log;

use crate::db::repositories::{
    ArtistRepository, ArtistSummaryRow, GroupRepository, GroupSummaryRow, Page, PaginatedResult,
};
use crate::db::seeder::DatasetSeeder;
use crate::dto::input::dataset::{DatasetDto, DatasetMetaDto};
use crate::dto::output::CollectionSummaryItem;
use crate::AppStore;

pub struct DatasetService<'a> {
    app: &'a AppHandle,
    store: &'a mut AppStore,
}

impl<'a> DatasetService<'a> {
    pub fn new(app: &'a AppHandle, store: &'a mut AppStore) -> Self {
        Self { app, store }
    }

    // -----------------------------------------------------------------------
    // Public API
    // -----------------------------------------------------------------------

    pub async fn sync(&mut self, force: bool) -> Result<bool, String> {
        let dataset_url = self.store.config.dataset_url.as_str();
        let dataset = Self::download(self.app, dataset_url).await?;
        let new_version = dataset.version.clone();
        let current_version = self.read_dataset_meta()?.version;

        let parsed_new_version = Version::parse(&new_version)
            .map_err(|e| format!("Failed to parse new dataset version: {}", e))?;
        let parsed_current_version = Version::parse(&current_version)
            .map_err(|e| format!("Failed to parse current dataset version: {}", e))?;

        if parsed_current_version >= parsed_new_version && !force {
            log::info!("Dataset is already up to date");
            return Ok(false);
        }

        let mut seeder = DatasetSeeder::new(&mut self.store.db_conn);
        let report = seeder
            .run(dataset)
            .map_err(|e| format!("Failed to seed database: {}", e))?;

        self.write_dataset_meta(parsed_new_version)?;
        log::info!("Dataset synchronization completed, report: {:?}", report);

        Ok(true)
    }

    pub fn get_summary(
        &mut self,
        page: Page,
        search: Option<&str>,
        agency_id: Option<&str>,
        include_photocards: bool,
        include_exclusive_items: bool,
    ) -> Result<PaginatedResult<CollectionSummaryItem>, diesel::result::Error> {
        let agency_ids: Option<Vec<String>> = agency_id.map(|id| vec![id.to_string()]);

        let mut groups = self.get_groups_summary(
            search,
            agency_ids.as_deref(),
            include_photocards,
            include_exclusive_items,
        )?;
        let mut artists = self.get_artists_summary(
            search,
            agency_ids.as_deref(),
            include_photocards,
            include_exclusive_items,
        )?;

        // Merge and convert
        let mut items: Vec<CollectionSummaryItem> = groups
            .drain(..)
            .map(|group| CollectionSummaryItem {
                id: group.id,
                kind: "group".to_string(),
                name: group.name,
                agency_name: group.agency_name,
                image_url: group.image_url,
                is_favorite: group.is_favorite,
                owned_count: group.owned_count,
                total_count: group.total_count,
            })
            .chain(artists.drain(..).map(|artist| CollectionSummaryItem {
                id: artist.id,
                kind: "solo".to_string(),
                name: artist.name,
                agency_name: artist.agency_name,
                image_url: artist.image_url,
                is_favorite: artist.is_favorite,
                owned_count: artist.owned_count,
                total_count: artist.total_count,
            }))
            .collect();

        // Sort: favorites first, then by name
        items.sort_by(|a, b| {
            b.is_favorite
                .cmp(&a.is_favorite)
                .then_with(|| a.name.cmp(&b.name))
        });

        let total = items.len() as i64;

        // In-memory pagination (data already loaded for cross-table sorting)
        let offset = page.offset() as usize;
        let limit = page.limit() as usize;
        let data: Vec<CollectionSummaryItem> = items.into_iter().skip(offset).take(limit).collect();

        Ok(PaginatedResult::new(data, page, total))
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    #[cfg(not(target_os = "android"))]
    async fn download(_app: &AppHandle, url: &str) -> Result<DatasetDto, String> {
        let response = reqwest::get(url)
            .await
            .map_err(|e| format!("Failed to fetch dataset: {}", e))?;

        response
            .json::<DatasetDto>()
            .await
            .map_err(|e| format!("Failed to parse dataset: {}", e))
    }

    #[cfg(target_os = "android")]
    async fn download(app: &AppHandle, url: &str) -> Result<DatasetDto, String> {
        use tauri_plugin_fs::FsExt;

        let path = app
            .path()
            .resolve(url, tauri::path::BaseDirectory::Resource)
            .expect("failed to resolve dataset path");

        let content = app
            .fs()
            .read_to_string(path)
            .expect("failed to read dataset content");

        match serde_json::from_str::<DatasetDto>(&content) {
            Ok(dataset) => Ok(dataset),
            Err(e) => {
                log::error!("Invalid JSON, returning default dataset: {}", e);
                Ok(DatasetDto::default())
            }
        }
    }

    fn dataset_meta_path(&self) -> Result<PathBuf, String> {
        let metadata_dir = self.app.path().app_data_dir().map_err(|e| e.to_string())?;

        Ok(metadata_dir.join("dataset_metadata.json"))
    }

    fn read_dataset_meta(&self) -> Result<DatasetMetaDto, String> {
        let metadata_path = self.dataset_meta_path()?;
        if !metadata_path.exists() {
            log::warn!("Dataset metadata file not found");
            return Ok(DatasetMetaDto::default());
        }

        let metadata_file = fs::read_to_string(&metadata_path)
            .map_err(|e| format!("Failed to read dataset metadata file: {}", e))?;

        match serde_json::from_str::<DatasetMetaDto>(&metadata_file) {
            Ok(dataset) => Ok(dataset),
            Err(e) => {
                log::error!("Invalid JSON, returning default dataset: {}", e);
                Ok(DatasetMetaDto::default())
            }
        }
    }

    fn write_dataset_meta(&self, version: Version) -> Result<(), String> {
        let metadata_path = self.dataset_meta_path()?;
        log::info!("dataset_metadata path: {}", metadata_path.to_str().unwrap());

        let meta = serde_json::json!(DatasetMetaDto::new(version));
        let json = serde_json::to_string_pretty(&meta)
            .map_err(|e| format!("Can not pretty string: {}", e))?;

        let mut file = File::create(&metadata_path)
            .map_err(|e| format!("Can not create dataset_metadata file: {}", e))?;

        file.write_all(json.as_bytes())
            .map_err(|e| format!("Can not write in dataset_metadata file: {}", e))?;

        Ok(())
    }

    fn get_groups_summary(
        &mut self,
        query: Option<&str>,
        agency_ids: Option<&[String]>,
        include_photocards: bool,
        include_exclusive_items: bool,
    ) -> Result<Vec<GroupSummaryRow>, diesel::result::Error> {
        GroupRepository::new(&mut self.store.db_conn).get_summary(
            false,
            query,
            agency_ids,
            include_photocards,
            include_exclusive_items,
        )
    }

    fn get_artists_summary(
        &mut self,
        query: Option<&str>,
        agency_ids: Option<&[String]>,
        include_photocards: bool,
        include_exclusive_items: bool,
    ) -> Result<Vec<ArtistSummaryRow>, diesel::result::Error> {
        ArtistRepository::new(&mut self.store.db_conn).get_summary(
            false,
            query,
            agency_ids,
            include_photocards,
            include_exclusive_items,
        )
    }
}
