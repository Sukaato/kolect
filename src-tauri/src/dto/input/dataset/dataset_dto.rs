// Struct racine qui désérialise dataset_v2.json en entier.
// La version est parsée en semver::Version pour permettre
// des vérifications de compatibilité au démarrage.

use super::{
    AgencyDto, AlbumDto, AlbumVersionDto, ArtistAliasDto, ArtistDto, DigipackDto, FanclubKitDto,
    GroupDto, GroupMemberDto, LightstickDto, PhotocardDto,
};
use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};

// ─── DatasetDto ───────────────────────────────────────────────────────────────

/// Représente le fichier dataset_v2.json complet.
/// Désérialisé une fois au démarrage, puis converti en `DatasetModels`
/// pour l'insertion en base.
#[derive(Debug, Deserialize, Default)]
pub struct DatasetDto {
    #[serde(default = "default_version_str")]
    pub version: String,
    pub agencies: Vec<AgencyDto>,
    pub groups: Vec<GroupDto>,
    pub artists: Vec<ArtistDto>,
    pub artist_aliases: Vec<ArtistAliasDto>,
    pub group_members: Vec<GroupMemberDto>,
    pub albums: Vec<AlbumDto>,
    pub album_versions: Vec<AlbumVersionDto>,
    pub digipacks: Vec<DigipackDto>,
    pub lightsticks: Vec<LightstickDto>,
    pub fanclub_kits: Vec<FanclubKitDto>,
    pub photocards: Vec<PhotocardDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetaDto {
    #[serde(default = "default_version_str")]
    pub version: String,

    #[serde(default = "default_fetched_at")]
    pub fetched_at: DateTime<Utc>,
}

impl DatasetMetaDto {
    pub fn new(version: Version) -> Self {
        Self {
            version: version.to_string(),
            fetched_at: default_fetched_at(),
        }
    }
}

impl Default for DatasetMetaDto {
    fn default() -> Self {
        Self::new(default_version())
    }
}

fn default_version() -> Version {
    Version::new(0, 0, 0)
}
fn default_version_str() -> String {
    default_version().to_string()
}

fn default_fetched_at() -> DateTime<Utc> {
    Utc::now() // millisecondes incluses automatiquement
}
