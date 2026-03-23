use crate::db::models::Album;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumDto {
    pub id: String,
    pub name: String,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
    /// Exactement l'un des deux doit être Some (contrainte applicative)
    pub group_id: Option<String>,
    pub artist_id: Option<String>,
}

impl From<AlbumDto> for Album {
    fn from(dto: AlbumDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            release_date: dto.release_date,
            region: dto.region,
            image_url: dto.image_url,
            group_id: dto.group_id,
            artist_id: dto.artist_id,
            is_deleted: 0,
        }
    }
}
