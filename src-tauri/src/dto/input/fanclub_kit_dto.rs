use crate::infrastructure::db::models::FanclubKit;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FanclubKitDto {
    pub id: String,
    pub name: String,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
    pub group_id: Option<String>,
    pub artist_id: Option<String>,
}

impl From<FanclubKitDto> for FanclubKit {
    fn from(dto: FanclubKitDto) -> Self {
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
