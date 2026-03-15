use crate::infrastructure::db::models::Artist;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ArtistDto {
    pub id: String,
    pub real_name: String,
    pub birth_date: Option<String>,
    pub image_url: Option<String>,
    pub solo_debut_date: Option<String>,
    pub solo_agency_id: Option<String>,
}

impl From<ArtistDto> for Artist {
    fn from(dto: ArtistDto) -> Self {
        Self {
            id: dto.id,
            real_name: dto.real_name,
            birth_date: dto.birth_date,
            image_url: dto.image_url,
            solo_debut_date: dto.solo_debut_date,
            solo_agency_id: dto.solo_agency_id,
            is_deleted: 0,
        }
    }
}
