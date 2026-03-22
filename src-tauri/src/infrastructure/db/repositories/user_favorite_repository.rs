use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::RepoResult;

pub struct UserFavoriteRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> UserFavoriteRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    // ─── Group ───────────────────────────────────────────────────────────────

    pub fn is_group_favorite(&mut self, g_id: &str) -> RepoResult<bool> {
        use crate::infrastructure::db::schema::user_favorites_groups::dsl::*;

        let count = user_favorites_groups
            .filter(group_id.eq(g_id))
            .count()
            .get_result::<i64>(self.conn)?;

        Ok(count > 0)
    }

    pub fn toggle_group_favorite(&mut self, g_id: &str) -> RepoResult<bool> {
        if self.is_group_favorite(g_id)? {
            self.remove_group_favorite(g_id)?;
            Ok(false)
        } else {
            self.add_group_favorite(g_id)?;
            Ok(true)
        }
    }

    fn add_group_favorite(&mut self, g_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::user_favorites_groups::dsl::*;

        diesel::insert_into(user_favorites_groups)
            .values(group_id.eq(g_id))
            .execute(self.conn)?;

        Ok(())
    }

    fn remove_group_favorite(&mut self, g_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::user_favorites_groups::dsl::*;

        diesel::delete(user_favorites_groups.filter(group_id.eq(g_id))).execute(self.conn)?;

        Ok(())
    }

    // ─── Artist ──────────────────────────────────────────────────────────────

    pub fn is_artist_favorite(&mut self, a_id: &str) -> RepoResult<bool> {
        use crate::infrastructure::db::schema::user_favorites_artists::dsl::*;

        let count = user_favorites_artists
            .filter(artist_id.eq(a_id))
            .count()
            .get_result::<i64>(self.conn)?;

        Ok(count > 0)
    }

    pub fn toggle_artist_favorite(&mut self, a_id: &str) -> RepoResult<bool> {
        if self.is_artist_favorite(a_id)? {
            self.remove_artist_favorite(a_id)?;
            Ok(false)
        } else {
            self.add_artist_favorite(a_id)?;
            Ok(true)
        }
    }

    fn add_artist_favorite(&mut self, a_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::user_favorites_artists::dsl::*;

        diesel::insert_into(user_favorites_artists)
            .values(artist_id.eq(a_id))
            .execute(self.conn)?;

        Ok(())
    }

    fn remove_artist_favorite(&mut self, a_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::user_favorites_artists::dsl::*;

        diesel::delete(user_favorites_artists.filter(artist_id.eq(a_id))).execute(self.conn)?;

        Ok(())
    }
}
