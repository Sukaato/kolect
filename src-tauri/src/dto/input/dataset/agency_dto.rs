use crate::infrastructure::db::models::Agency;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AgencyDto {
    pub id: String,
    pub name: String,
    pub country: String,
    pub image_url: Option<String>,
}

impl From<AgencyDto> for Agency {
    fn from(dto: AgencyDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            country: dto.country,
            image_url: dto.image_url,
            is_deleted: 0,
        }
    }
}
