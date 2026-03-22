use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CollectibleType {
    Album,
    AlbumVersion,
    Digipack,
    Lightstick,
    FanclubKit,
    Photocard,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateItemDto {
    pub item_type: CollectibleType,
    pub item_id: String,
    pub owned_count: u32,
    pub signed_count: u32,
}
