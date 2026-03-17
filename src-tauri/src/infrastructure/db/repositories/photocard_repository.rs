// src-tauri/src/infrastructure/db/repositories/photocard_repository.rs

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Photocard;
use crate::infrastructure::db::schema::photocards::dsl::*;

pub struct PhotocardRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> PhotocardRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

impl<'a> Repository<Photocard> for PhotocardRepository<'a> {
    fn insert(&mut self, item: Photocard) -> RepoResult<Photocard> {
        diesel::insert_into(photocards)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Photocard>> {
        Ok(photocards
            .filter(id.eq(record_id))
            .first::<Photocard>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Photocard>> {
        let total = photocards
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = photocards
            .filter(is_deleted.eq(0))
            .order(release_date.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Photocard>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Photocard) -> RepoResult<Photocard> {
        diesel::update(photocards.filter(id.eq(&item.id)))
            .set((
                artist_id.eq(&item.artist_id),
                album_id.eq(&item.album_id),
                album_version_id.eq(&item.album_version_id),
                digipack_id.eq(&item.digipack_id),
                release_date.eq(&item.release_date),
                region.eq(&item.region),
                image_url.eq(&item.image_url),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        diesel::update(photocards.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
