use crate::db::models::Digipack;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DigipackDto {
    pub id: String,
    pub album_id: String,
    /// None pour les digipacks de groupe sans membre spécifique
    pub artist_id: Option<String>,
    pub name: String,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
}

impl From<DigipackDto> for Digipack {
    fn from(dto: DigipackDto) -> Self {
        Self {
            id: dto.id,
            album_id: dto.album_id,
            artist_id: dto.artist_id,
            name: dto.name,
            release_date: dto.release_date,
            region: dto.region,
            image_url: dto.image_url,
            is_deleted: 0,
        }
    }
}
