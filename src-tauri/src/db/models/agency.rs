use crate::db::schema::agencies;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Serialize)]
#[diesel(table_name = agencies)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Agency {
    pub id: String,
    pub name: String,
    pub country: String,
    pub image_url: Option<String>,
    /// SQLite stocke les booleans comme INTEGER (0/1).
    /// Utiliser `.is_deleted()` pour obtenir un bool.
    pub is_deleted: i32,
}

impl Agency {
    pub fn is_deleted(&self) -> bool {
        self.is_deleted != 0
    }
}
