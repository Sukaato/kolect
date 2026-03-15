use crate::infrastructure::db::models::Photocard;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PhotocardDto {
    pub id: String,
    pub name: String,
    /// None pour les photocards de groupe (sans membre spécifique)
    pub artist_id: Option<String>,
    pub album_id: Option<String>,
    pub album_version_id: Option<String>,
    pub digipack_id: Option<String>,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
}

impl From<PhotocardDto> for Photocard {
    fn from(dto: PhotocardDto) -> Self {
        Self {
            id: dto.id,
            artist_id: dto.artist_id,
            album_id: dto.album_id,
            album_version_id: dto.album_version_id,
            digipack_id: dto.digipack_id,
            release_date: dto.release_date,
            region: dto.region,
            image_url: dto.image_url,
            is_deleted: 0,
        }
    }
}
