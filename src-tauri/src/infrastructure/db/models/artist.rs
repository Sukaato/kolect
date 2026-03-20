use crate::infrastructure::db::models::Agency;
use crate::infrastructure::db::schema::artists;
use diesel::prelude::*;
use serde::Serialize;

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize,
)]
#[diesel(table_name = artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Agency, foreign_key = solo_agency_id))]
pub struct Artist {
    pub id: String,
    pub real_name: String,
    pub birth_date: Option<String>,
    pub image_url: Option<String>,
    pub solo_debut_date: Option<String>,
    pub solo_agency_id: Option<String>,
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_deleted()` pour obtenir un bool.
    pub is_deleted: i32,
}

impl Artist {
    pub fn is_deleted(&self) -> bool {
        self.is_deleted != 0
    }

    /// Vrai si l'artiste a une carrière solo
    pub fn has_solo_career(&self) -> bool {
        self.solo_debut_date.is_some()
    }
}
