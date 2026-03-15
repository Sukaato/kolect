use crate::infrastructure::db::models::{Artist, Group};
use crate::infrastructure::db::schema::lightsticks;
use diesel::prelude::*;
use serde::Serialize;

/// Exactement un de group_id / artist_id doit être Some (contrainte applicative).
/// Ex: lightstick BLACKPINK → group_id, lightstick Lisa → artist_id
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize)]
#[diesel(table_name = lightsticks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Artist))]
pub struct Lightstick {
    pub id: String,
    pub group_id: Option<String>,
    pub artist_id: Option<String>,
    pub name: String,
    pub version: String,
    pub release_date: String,
    pub image_url: Option<String>,
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_primary()` pour obtenir un bool.
    pub is_deleted: i32,
}

impl Lightstick {
    pub fn is_deleted(&self) -> bool {
        self.is_deleted != 0
    }

    pub fn is_group_lightstick(&self) -> bool {
        self.group_id.is_some()
    }

    pub fn is_solo_lightstick(&self) -> bool {
        self.artist_id.is_some()
    }
}
