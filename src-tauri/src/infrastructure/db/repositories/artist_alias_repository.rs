use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::ArtistAlias;

pub struct ArtistAliasRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> ArtistAliasRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Retourne tous les aliases actifs pour une liste d'artistes.
    pub fn find_by_artist_ids(&mut self, ids: &[String]) -> RepoResult<Vec<ArtistAlias>> {
        use crate::infrastructure::db::schema::artist_aliases::dsl::*;

        Ok(artist_aliases
            .filter(artist_id.eq_any(ids))
            .filter(is_deleted.eq(0))
            .load::<ArtistAlias>(self.conn)?)
    }
}

impl<'a> Repository<ArtistAlias> for ArtistAliasRepository<'a> {
    fn insert(&mut self, item: ArtistAlias) -> RepoResult<ArtistAlias> {
        use crate::infrastructure::db::schema::artist_aliases::dsl::*;

        diesel::insert_into(artist_aliases)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<ArtistAlias>> {
        use crate::infrastructure::db::schema::artist_aliases::dsl::*;

        Ok(artist_aliases
            .filter(id.eq(record_id))
            .first::<ArtistAlias>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<ArtistAlias>> {
        use crate::infrastructure::db::schema::artist_aliases::dsl::*;

        let total = artist_aliases
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = artist_aliases
            .filter(is_deleted.eq(0))
            .order(artist_id.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<ArtistAlias>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: ArtistAlias) -> RepoResult<ArtistAlias> {
        use crate::infrastructure::db::schema::artist_aliases::dsl::*;

        diesel::update(artist_aliases.filter(id.eq(&item.id)))
            .set((
                artist_id.eq(&item.artist_id),
                name.eq(&item.name),
                kind.eq(&item.kind),
                is_primary.eq(&item.is_primary),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::artist_aliases::dsl::*;

        diesel::update(artist_aliases.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}