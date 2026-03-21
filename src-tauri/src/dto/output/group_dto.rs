use serde::Serialize;

use super::artist_dto::ArtistWithAliasesDto;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupOutputDto {
    pub id: String,
    pub name: String,
    pub debut_date: String,
    pub fandom_name: Option<String>,
    pub image_url: Option<String>,
    pub agency_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupDetailDto {
    pub group: GroupOutputDto,
    pub members: Vec<ArtistWithAliasesDto>,
}