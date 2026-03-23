use crate::db::models::{Artist, Group};
use crate::db::schema::fanclub_kits;
use diesel::prelude::*;
use serde::Serialize;

/// Exactement un de group_id / artist_id doit être Some (contrainte applicative).
#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize,
)]
#[diesel(table_name = fanclub_kits)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Artist))]
pub struct FanclubKit {
    pub id: String,
    pub group_id: Option<String>,
    pub artist_id: Option<String>,
    pub name: String,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_primary()` pour obtenir un bool.
    pub is_deleted: i32,
}

impl FanclubKit {
    pub fn is_deleted(&self) -> bool {
        self.is_deleted != 0
    }

    pub fn is_group_kit(&self) -> bool {
        self.group_id.is_some()
    }

    pub fn is_solo_kit(&self) -> bool {
        self.artist_id.is_some()
    }
}
