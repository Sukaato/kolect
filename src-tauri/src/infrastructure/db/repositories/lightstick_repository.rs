// src-tauri/src/infrastructure/db/repositories/lightstick_repository.rs

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Lightstick;
use crate::infrastructure::db::schema::lightsticks::dsl::*;

pub struct LightstickRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> LightstickRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

impl<'a> Repository<Lightstick> for LightstickRepository<'a> {
    fn insert(&mut self, item: Lightstick) -> RepoResult<Lightstick> {
        diesel::insert_into(lightsticks)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Lightstick>> {
        Ok(lightsticks
            .filter(id.eq(record_id))
            .first::<Lightstick>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Lightstick>> {
        let total = lightsticks
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = lightsticks
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Lightstick>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Lightstick) -> RepoResult<Lightstick> {
        diesel::update(lightsticks.filter(id.eq(&item.id)))
            .set((
                group_id.eq(&item.group_id),
                artist_id.eq(&item.artist_id),
                name.eq(&item.name),
                version.eq(&item.version),
                release_date.eq(&item.release_date),
                image_url.eq(&item.image_url),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        diesel::update(lightsticks.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
