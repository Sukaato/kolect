// src-tauri/src/infrastructure/db/repositories/artist_repository.rs

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Artist;
use crate::infrastructure::db::schema::artists::dsl::*;

pub struct ArtistRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> ArtistRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

impl<'a> Repository<Artist> for ArtistRepository<'a> {
    fn insert(&mut self, item: Artist) -> RepoResult<Artist> {
        diesel::insert_into(artists)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Artist>> {
        Ok(artists
            .filter(id.eq(record_id))
            .first::<Artist>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Artist>> {
        let total = artists
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = artists
            .filter(is_deleted.eq(0))
            .order(real_name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Artist>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Artist) -> RepoResult<Artist> {
        diesel::update(artists.filter(id.eq(&item.id)))
            .set((
                real_name.eq(&item.real_name),
                birth_date.eq(&item.birth_date),
                image_url.eq(&item.image_url),
                solo_debut_date.eq(&item.solo_debut_date),
                solo_agency_id.eq(&item.solo_agency_id),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        diesel::update(artists.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
