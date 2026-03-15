use crate::infrastructure::db::models::{Artist, Group};
use crate::infrastructure::db::schema::group_members;
use diesel::prelude::*;
use serde::Serialize;

/// Table de jonction Artist ↔ Group avec métadonnées d'appartenance.
/// Clé primaire composite : (artist_id, group_id)
/// leave_date IS NULL → membre encore actif
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize)]
#[diesel(table_name = group_members)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(artist_id, group_id))]
#[diesel(belongs_to(Artist))]
#[diesel(belongs_to(Group))]
pub struct GroupMember {
    pub artist_id: String,
    pub group_id: String,
    pub roles: String,
    pub join_date: Option<String>,
    pub leave_date: Option<String>,
}

impl GroupMember {
    /// Membre encore actif dans le groupe
    pub fn is_active(&self) -> bool {
        self.leave_date.is_none()
    }

    pub fn roles(&self) -> Vec<&str> {
        self.roles.split(',').map(str::trim).collect()
    }
}
