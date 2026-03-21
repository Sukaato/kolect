use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Agency;
use crate::infrastructure::db::schema::agencies::dsl::*;

pub struct AgencyRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> AgencyRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

impl<'a> Repository<Agency> for AgencyRepository<'a> {
    fn insert(&mut self, item: Agency) -> RepoResult<Agency> {
        diesel::insert_into(agencies)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Agency>> {
        Ok(agencies
            .filter(id.eq(record_id))
            .first::<Agency>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Agency>> {
        let total = agencies
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = agencies
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Agency>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Agency) -> RepoResult<Agency> {
        diesel::update(agencies.filter(id.eq(&item.id)))
            .set((
                name.eq(&item.name),
                country.eq(&item.country),
                image_url.eq(&item.image_url),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        diesel::update(agencies.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
