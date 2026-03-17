// src-tauri/src/infrastructure/db/repositories/digipack_repository.rs

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Digipack;
use crate::infrastructure::db::schema::digipacks::dsl::*;

pub struct DigipackRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> DigipackRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

impl<'a> Repository<Digipack> for DigipackRepository<'a> {
    fn insert(&mut self, item: Digipack) -> RepoResult<Digipack> {
        diesel::insert_into(digipacks)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Digipack>> {
        Ok(digipacks
            .filter(id.eq(record_id))
            .first::<Digipack>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Digipack>> {
        let total = digipacks
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = digipacks
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Digipack>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Digipack) -> RepoResult<Digipack> {
        diesel::update(digipacks.filter(id.eq(&item.id)))
            .set((
                album_id.eq(&item.album_id),
                artist_id.eq(&item.artist_id),
                name.eq(&item.name),
                release_date.eq(&item.release_date),
                region.eq(&item.region),
                image_url.eq(&item.image_url),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        diesel::update(digipacks.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
