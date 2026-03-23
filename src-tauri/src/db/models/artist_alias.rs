use crate::db::models::Artist;
use crate::db::schema::artist_aliases;
use diesel::prelude::*;
use serde::Serialize;

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize,
)]
#[diesel(table_name = artist_aliases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Artist))]
pub struct ArtistAlias {
    pub id: String,
    pub artist_id: String,
    pub name: String,
    pub kind: String, // voir AliasKind
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_primary()` pour obtenir un bool.
    pub is_primary: i32,
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_primary()` pour obtenir un bool.
    pub is_deleted: i32,
}

/// Enum applicatif pour le champ `kind`.
/// Correspond au CHECK constraint en DB.
#[derive(Debug, Clone, PartialEq)]
pub enum AliasKind {
    /// Nom de scène dans le groupe (ex: Jimin)
    GroupStage,
    /// Nom de scène en solo (ex: JIMIN)
    SoloStage,
    /// Nom dans la langue d'origine (ex: 지민)
    Original,
}

impl AliasKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            AliasKind::GroupStage => "group_stage",
            AliasKind::SoloStage => "solo_stage",
            AliasKind::Original => "original",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "group_stage" => Some(AliasKind::GroupStage),
            "solo_stage" => Some(AliasKind::SoloStage),
            "original" => Some(AliasKind::Original),
            _ => None,
        }
    }
}

impl ArtistAlias {
    pub fn is_deleted(&self) -> bool {
        self.is_deleted != 0
    }

    /// Convertit le champ SQLite INTEGER en bool.
    pub fn is_primary(&self) -> bool {
        self.is_primary != 0
    }

    pub fn kind_enum(&self) -> Option<AliasKind> {
        AliasKind::from_str(&self.kind)
    }

    pub fn is_group_stage(&self) -> bool {
        self.kind == AliasKind::GroupStage.as_str()
    }

    pub fn is_solo_stage(&self) -> bool {
        self.kind == AliasKind::SoloStage.as_str()
    }

    pub fn is_original(&self) -> bool {
        self.kind == AliasKind::Original.as_str()
    }
}
