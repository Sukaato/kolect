use crate::infrastructure::db::models::{Artist, Group};
use crate::infrastructure::db::schema::{user_favorites_artists, user_favorites_groups};
use diesel::prelude::*;
use serde::Serialize;

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize,
)]
#[diesel(table_name = user_favorites_groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(group_id))]
#[diesel(belongs_to(Group))]
pub struct UserFavoriteGroup {
    pub group_id: Option<String>,
}

#[derive(
    Debug, Clone, Queryable, Selectable, Identifiable, Insertable, Associations, Serialize,
)]
#[diesel(table_name = user_favorites_artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(artist_id))]
#[diesel(belongs_to(Artist))]
pub struct UserFavoriteArtist {
    pub artist_id: Option<String>,
}
