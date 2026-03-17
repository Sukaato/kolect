// src-tauri/src/infrastructure/db/repositories/album_repository.rs

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Album;
use crate::infrastructure::db::schema::albums::dsl::*;

pub struct AlbumRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> AlbumRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

impl<'a> Repository<Album> for AlbumRepository<'a> {
    fn insert(&mut self, item: Album) -> RepoResult<Album> {
        diesel::insert_into(albums)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Album>> {
        Ok(albums
            .filter(id.eq(record_id))
            .first::<Album>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Album>> {
        let total = albums
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = albums
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Album>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Album) -> RepoResult<Album> {
        diesel::update(albums.filter(id.eq(&item.id)))
            .set((
                name.eq(&item.name),
                release_date.eq(&item.release_date),
                region.eq(&item.region),
                image_url.eq(&item.image_url),
                group_id.eq(&item.group_id),
                artist_id.eq(&item.artist_id),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        diesel::update(albums.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
