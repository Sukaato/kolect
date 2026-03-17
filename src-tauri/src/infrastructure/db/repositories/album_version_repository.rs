// src-tauri/src/infrastructure/db/repositories/album_version_repository.rs

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::AlbumVersion;
use crate::infrastructure::db::schema::album_versions::dsl::*;

pub struct AlbumVersionRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> AlbumVersionRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

impl<'a> Repository<AlbumVersion> for AlbumVersionRepository<'a> {
    fn insert(&mut self, item: AlbumVersion) -> RepoResult<AlbumVersion> {
        diesel::insert_into(album_versions)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<AlbumVersion>> {
        Ok(album_versions
            .filter(id.eq(record_id))
            .first::<AlbumVersion>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<AlbumVersion>> {
        let total = album_versions
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = album_versions
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<AlbumVersion>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: AlbumVersion) -> RepoResult<AlbumVersion> {
        diesel::update(album_versions.filter(id.eq(&item.id)))
            .set((
                album_id.eq(&item.album_id),
                name.eq(&item.name),
                format.eq(&item.format),
                release_date.eq(&item.release_date),
                region.eq(&item.region),
                image_url.eq(&item.image_url),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        diesel::update(album_versions.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
