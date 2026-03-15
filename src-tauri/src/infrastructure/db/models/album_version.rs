use crate::infrastructure::db::models::Album;
use crate::infrastructure::db::schema::album_versions;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize)]
#[diesel(table_name = album_versions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Album))]
pub struct AlbumVersion {
    pub id: String,
    pub album_id: String,
    pub name: String,
    pub format: String, // voir VersionFormat
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_deleted()` pour obtenir un bool.
    pub is_deleted: i32,
}

/// Enum applicatif pour le champ `format`.
/// Correspond au CHECK constraint en DB.
#[derive(Debug, Clone, PartialEq)]
pub enum VersionFormat {
    /// CD standard
    Cd,
    /// Extended Play
    Ep,
    /// Album complet
    Album,
    /// Mini album
    MiniAlbum,
    /// Vinyle
    Vinyl,
    /// Kit album (format physique sans CD)
    Kit,
}

impl VersionFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            VersionFormat::Cd => "cd",
            VersionFormat::Ep => "ep",
            VersionFormat::Album => "album",
            VersionFormat::MiniAlbum => "mini_album",
            VersionFormat::Vinyl => "vinyl",
            VersionFormat::Kit => "kit",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cd" => Some(VersionFormat::Cd),
            "ep" => Some(VersionFormat::Ep),
            "album" => Some(VersionFormat::Album),
            "mini_album" => Some(VersionFormat::MiniAlbum),
            "vinyl" => Some(VersionFormat::Vinyl),
            "kit" => Some(VersionFormat::Kit),
            _ => None,
        }
    }
}

impl AlbumVersion {
    pub fn is_deleted(&self) -> bool {
        self.is_deleted != 0
    }

    pub fn format_enum(&self) -> Option<VersionFormat> {
        VersionFormat::from_str(&self.format)
    }

    pub fn is_vinyl(&self) -> bool {
        self.format == VersionFormat::Vinyl.as_str()
    }
}
