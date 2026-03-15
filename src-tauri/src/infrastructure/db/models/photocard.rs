use crate::infrastructure::db::models::{Album, AlbumVersion, Artist, Digipack};
use crate::infrastructure::db::schema::photocards;
use diesel::prelude::*;
use serde::Serialize;

/// Exactement une des trois FK doit être Some (contrainte applicative) :
///   album_id seul           → photocard random toutes versions
///   album_version_id seul   → photocard d'une version spécifique
///   digipack_id seul        → photocard incluse dans un digipack
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize)]
#[diesel(table_name = photocards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Artist))]
#[diesel(belongs_to(Album))]
#[diesel(belongs_to(AlbumVersion))]
#[diesel(belongs_to(Digipack))]
pub struct Photocard {
    pub id: String,
    pub artist_id: Option<String>,
    pub album_id: Option<String>,
    pub album_version_id: Option<String>,
    pub digipack_id: Option<String>,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_primary()` pour obtenir un bool.
    pub is_deleted: i32,
}

impl Photocard {
    pub fn is_deleted(&self) -> bool {
        self.is_deleted != 0
    }

    pub fn kind(&self) -> PhotocardKind {
        if self.digipack_id.is_some() {
            PhotocardKind::Digipack
        } else if self.album_version_id.is_some() {
            PhotocardKind::AlbumVersion
        } else {
            PhotocardKind::AlbumRandom
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PhotocardKind {
    /// Photocard random incluse dans toutes les versions de l'album
    AlbumRandom,
    /// Photocard spécifique à une version d'album
    AlbumVersion,
    /// Photocard incluse dans un digipack membre
    Digipack,
}
