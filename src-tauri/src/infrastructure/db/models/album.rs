use crate::infrastructure::db::models::{Artist, Group};
use crate::infrastructure::db::schema::albums;
use diesel::prelude::*;
use serde::Serialize;

/// Exactement un de group_id / artist_id doit être Some
/// (album de groupe OU album solo — contrainte applicative)
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Artist))]
pub struct Album {
    pub id: String,
    pub name: String,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
    pub group_id: Option<String>,
    pub artist_id: Option<String>,
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_primary()` pour obtenir un bool.
    pub is_deleted: i32,
}

impl Album {
    pub fn is_deleted(&self) -> bool {
        self.is_deleted != 0
    }

    pub fn is_group_album(&self) -> bool {
        self.group_id.is_some()
    }

    pub fn is_solo_album(&self) -> bool {
        self.artist_id.is_some()
    }
}
