use crate::infrastructure::db::models::AlbumVersion;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumVersionDto {
    pub id: String,
    pub album_id: String,
    pub name: String,
    /// "cd" | "ep" | "album" | "mini_album" | "vinyl" | "kit"
    pub format: String,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
}

impl From<AlbumVersionDto> for AlbumVersion {
    fn from(dto: AlbumVersionDto) -> Self {
        Self {
            id: dto.id,
            album_id: dto.album_id,
            name: dto.name,
            format: dto.format,
            release_date: dto.release_date,
            region: dto.region,
            image_url: dto.image_url,
            is_deleted: 0,
        }
    }
}
