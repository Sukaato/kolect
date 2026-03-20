use crate::infrastructure::db::models::{Album, Artist};
use crate::infrastructure::db::schema::digipacks;
use diesel::prelude::*;
use serde::Serialize;

/// Un digipack est lié à un album ET à un artiste spécifique.
/// Ex : digipack Lisa de Born Pink → album_id=born_pink, artist_id=lisa
#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize,
)]
#[diesel(table_name = digipacks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Album))]
#[diesel(belongs_to(Artist))]
pub struct Digipack {
    pub id: String,
    pub album_id: String,
    pub artist_id: Option<String>,
    pub name: String,
    pub release_date: String,
    pub region: String,
    pub image_url: Option<String>,
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_primary()` pour obtenir un bool.
    pub is_deleted: i32,
}

impl Digipack {
    pub fn is_deleted(&self) -> bool {
        self.is_deleted != 0
    }
}
