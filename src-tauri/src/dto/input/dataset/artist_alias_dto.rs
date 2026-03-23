use crate::db::models::ArtistAlias;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ArtistAliasDto {
    pub id: String,
    pub artist_id: String,
    pub name: String,
    /// "group_stage" | "solo_stage" | "original"
    pub kind: String,
    /// bool dans le JSON → i32 dans le model SQLite (0/1)
    pub is_primary: bool,
}

impl From<ArtistAliasDto> for ArtistAlias {
    fn from(dto: ArtistAliasDto) -> Self {
        Self {
            id: dto.id,
            artist_id: dto.artist_id,
            name: dto.name,
            kind: dto.kind,
            is_primary: dto.is_primary as i32,
            is_deleted: 0,
        }
    }
}
