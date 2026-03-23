use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::RepoResult;
use crate::db::models::ArtistAlias;

pub struct ArtistAliasRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> ArtistAliasRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns all active aliases for a list of artist IDs.
    pub fn find_by_artist_ids(&mut self, ids: &[String]) -> RepoResult<Vec<ArtistAlias>> {
        use crate::db::schema::artist_aliases::dsl::*;

        Ok(artist_aliases
            .filter(artist_id.eq_any(ids))
            .filter(is_deleted.eq(0))
            .load::<ArtistAlias>(self.conn)?)
    }
}
