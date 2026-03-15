// Struct racine qui désérialise dataset_v2.json en entier.
// La version est parsée en semver::Version pour permettre
// des vérifications de compatibilité au démarrage.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use semver::Version;
use super::{
    AgencyDto,
    AlbumDto,
    AlbumVersionDto,
    ArtistDto,
    ArtistAliasDto,
    DigipackDto,
    FanclubKitDto,
    GroupDto,
    GroupMemberDto,
    LightstickDto,
    PhotocardDto,
};

// ─── Désérialiseur custom pour semver::Version ────────────────────────────────

fn deserialize_version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Version::parse(&s).map_err(serde::de::Error::custom)
}

// ─── DatasetDto ───────────────────────────────────────────────────────────────

/// Représente le fichier dataset_v2.json complet.
/// Désérialisé une fois au démarrage, puis converti en `DatasetModels`
/// pour l'insertion en base.
#[derive(Debug, Deserialize)]
pub struct DatasetDto {
    #[serde(deserialize_with = "deserialize_version")]
    pub version:        Version,
    pub agencies:       Vec<AgencyDto>,
    pub groups:         Vec<GroupDto>,
    pub artists:        Vec<ArtistDto>,
    pub artist_aliases: Vec<ArtistAliasDto>,
    pub group_members:  Vec<GroupMemberDto>,
    pub albums:         Vec<AlbumDto>,
    pub album_versions: Vec<AlbumVersionDto>,
    pub digipacks:      Vec<DigipackDto>,
    pub lightsticks:    Vec<LightstickDto>,
    pub fanclub_kits:   Vec<FanclubKitDto>,
    pub photocards:     Vec<PhotocardDto>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetaDto {
    #[serde(default = "default_version")]
    pub version: Version,

    #[serde(default = "default_fetched_at")]
    pub fetched_at: DateTime<Utc>,
}

impl DatasetMetaDto {
    pub fn new(version: Version) -> Self {
        Self {
            version,
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

fn default_fetched_at() -> DateTime<Utc> {
    Utc::now() // millisecondes incluses automatiquement
}
