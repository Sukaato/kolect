use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSummaryDto {
    pub album_id: String,
    pub name: String,
    pub release_date: String,
    pub image_url: Option<String>,
    // Versions
    pub versions_owned_count: i64,
    pub versions_total_count: i64,
    // Digipacks
    pub digipacks_owned_count: i64,
    pub digipacks_total_count: i64,
    // Photocards (always returned, frontend decides whether to show)
    pub photocards_owned_count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumDetailDto {
    pub album_id: String,
    pub name: String,
    pub release_date: String,
    pub image_url: Option<String>,
    pub group_id: Option<String>,
    pub artist_id: Option<String>,
    // Versions progress
    pub versions_owned_count: i64,
    pub versions_total_count: i64,
    // Digipacks progress
    pub digipacks_owned_count: i64,
    pub digipacks_total_count: i64,
    // Photocards progress (included only when setting is enabled)
    pub photocards_owned_count: i64,
    pub photocards_total_count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumVersionItemDto {
    pub id: String,
    pub name: String,
    pub format: String,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
    pub owned_count: i64,
    pub has_signed: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DigipackItemDto {
    pub id: String,
    pub name: String,
    pub artist_id: Option<String>,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
    pub owned_count: i64,
    pub has_signed: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotocardItemDto {
    pub id: String,
    pub artist_id: Option<String>,
    pub album_version_id: Option<String>,
    pub digipack_id: Option<String>,
    pub region: String,
    pub image_url: Option<String>,
    pub owned_count: i64,
    pub has_signed: bool,
}
