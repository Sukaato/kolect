use crate::infrastructure::db::models::Agency;
use crate::infrastructure::db::schema::groups;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize)]
#[diesel(table_name = groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Agency))]
pub struct Group {
    pub id: String,
    pub name: String,
    pub debut_date: String,
    pub fandom_name: Option<String>,
    pub image_url: Option<String>,
    pub agency_id: String,
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_deleted()` pour obtenir un bool.
    pub is_deleted: i32,
}

impl Group {
    pub fn is_deleted(&self) -> bool {
        self.is_deleted != 0
    }
}