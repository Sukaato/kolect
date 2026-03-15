use crate::infrastructure::db::models::{
    Album, AlbumVersion, Digipack, FanclubKit, Lightstick, Photocard,
};
use crate::infrastructure::db::schema::user_collection;
use diesel::prelude::*;
use serde::Serialize;

/// Un exemplaire physique possédé par l'utilisateur.
/// Exactement une des 6 FK doit être Some (contrainte applicative).
/// Plusieurs lignes avec la même FK = doublons physiques autorisés.
/// is_signed pertinent pour : album_version, digipack, photocard.
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize)]
#[diesel(table_name = user_collection)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Album))]
#[diesel(belongs_to(AlbumVersion))]
#[diesel(belongs_to(Digipack))]
#[diesel(belongs_to(Lightstick))]
#[diesel(belongs_to(FanclubKit))]
#[diesel(belongs_to(Photocard))]
pub struct UserCollection {
    pub id: String,
    pub album_id: Option<String>,
    pub album_version_id: Option<String>,
    pub digipack_id: Option<String>,
    pub lightstick_id: Option<String>,
    pub fanclub_kit_id: Option<String>,
    pub photocard_id: Option<String>,
    pub acquired_at: String,
    pub notes: Option<String>,
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_signed()` pour obtenir un bool.
    pub is_signed: i32,
}

impl UserCollection {
    pub fn is_signed(&self) -> bool {
        self.is_signed != 0
    }

    /// Valide qu'exactement une FK est renseignée.
    pub fn is_valid(&self) -> bool {
        [
            self.album_id.is_some(),
            self.album_version_id.is_some(),
            self.digipack_id.is_some(),
            self.lightstick_id.is_some(),
            self.fanclub_kit_id.is_some(),
            self.photocard_id.is_some(),
        ]
        .iter()
        .filter(|&&v| v)
        .count()
            == 1
    }

    pub fn item_kind(&self) -> UserCollectionItemKind {
        if self.album_id.is_some() {
            return UserCollectionItemKind::Album;
        }
        if self.album_version_id.is_some() {
            return UserCollectionItemKind::AlbumVersion;
        }
        if self.digipack_id.is_some() {
            return UserCollectionItemKind::Digipack;
        }
        if self.lightstick_id.is_some() {
            return UserCollectionItemKind::Lightstick;
        }
        if self.fanclub_kit_id.is_some() {
            return UserCollectionItemKind::FanclubKit;
        }
        UserCollectionItemKind::Photocard
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserCollectionItemKind {
    Album,
    AlbumVersion,
    Digipack,
    Lightstick,
    FanclubKit,
    Photocard,
}
