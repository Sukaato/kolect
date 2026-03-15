use crate::infrastructure::db::models::Group;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GroupDto {
    pub id: String,
    pub name: String,
    pub debut_date: String,
    pub agency_id: String,
    pub fandom_name: Option<String>,
    pub image_url: Option<String>,
}

impl From<GroupDto> for Group {
    fn from(dto: GroupDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            debut_date: dto.debut_date,
            agency_id: dto.agency_id,
            fandom_name: dto.fandom_name,
            image_url: dto.image_url,
            is_deleted: 0,
        }
    }
}
