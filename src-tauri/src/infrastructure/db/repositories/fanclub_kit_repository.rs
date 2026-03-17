// src-tauri/src/infrastructure/db/repositories/fanclub_kit_repository.rs

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::FanclubKit;
use crate::infrastructure::db::schema::fanclub_kits::dsl::*;

pub struct FanclubKitRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> FanclubKitRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

impl<'a> Repository<FanclubKit> for FanclubKitRepository<'a> {
    fn insert(&mut self, item: FanclubKit) -> RepoResult<FanclubKit> {
        diesel::insert_into(fanclub_kits)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<FanclubKit>> {
        Ok(fanclub_kits
            .filter(id.eq(record_id))
            .first::<FanclubKit>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<FanclubKit>> {
        let total = fanclub_kits
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = fanclub_kits
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<FanclubKit>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: FanclubKit) -> RepoResult<FanclubKit> {
        diesel::update(fanclub_kits.filter(id.eq(&item.id)))
            .set((
                group_id.eq(&item.group_id),
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
        diesel::update(fanclub_kits.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
