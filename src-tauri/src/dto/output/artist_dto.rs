use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistOutputDto {
    pub id: String,
    pub real_name: String,
    pub birth_date: Option<String>,
    pub image_url: Option<String>,
    pub solo_debut_date: Option<String>,
    pub solo_agency_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAliasOutputDto {
    pub id: String,
    pub artist_id: String,
    pub name: String,
    pub kind: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistWithAliasesDto {
    pub artist: ArtistOutputDto,
    pub aliases: Vec<ArtistAliasOutputDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LightstickItemDto {
    pub id: String,
    pub group_id: Option<String>,
    pub artist_id: Option<String>,
    pub name: String,
    pub version: String,
    pub release_date: String,
    pub image_url: Option<String>,
    pub owned_count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FanclubKitItemDto {
    pub id: String,
    pub group_id: Option<String>,
    pub artist_id: Option<String>,
    pub name: String,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
    pub owned_count: i64,
}
