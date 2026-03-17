// src-tauri/src/infrastructure/db/repositories/group_repository.rs

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::infrastructure::db::models::Group;
use crate::infrastructure::db::schema::groups::dsl::*;
use super::{Page, PaginatedResult, Repository, RepositoryError, RepoResult};

pub struct GroupRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> GroupRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }
}

impl<'a> Repository<Group> for GroupRepository<'a> {
    fn insert(&mut self, item: Group) -> RepoResult<Group> {
        diesel::insert_into(groups)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Group>> {
        Ok(groups
            .filter(id.eq(record_id))
            .first::<Group>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Group>> {
        let total = groups
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = groups
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Group>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Group) -> RepoResult<Group> {
        diesel::update(groups.filter(id.eq(&item.id)))
            .set((                name.eq(&item.name),
                debut_date.eq(&item.debut_date),
                agency_id.eq(&item.agency_id),
                fandom_name.eq(&item.fandom_name),
                image_url.eq(&item.image_url),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        diesel::update(groups.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}