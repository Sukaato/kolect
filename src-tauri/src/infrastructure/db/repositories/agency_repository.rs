use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Agency;

pub struct AgencyRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> AgencyRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns all non-deleted agencies, ordered by name.
    pub fn find_all_active(&mut self) -> RepoResult<Vec<Agency>> {
        use crate::infrastructure::db::schema::agencies::dsl::*;

        Ok(agencies
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .load::<Agency>(self.conn)?)
    }

    /// Returns agencies that have at least one item owned in user_collection,
    /// ordered by name. Uses the collection_agencies_view maintained by the database.
    pub fn find_for_collection(&mut self) -> RepoResult<Vec<Agency>> {
        #[derive(QueryableByName)]
        struct Row {
            #[diesel(sql_type = diesel::sql_types::Text)]
            agency_id: String,
            #[diesel(sql_type = diesel::sql_types::Text)]
            agency_name: String,
        }

        let rows = diesel::sql_query(
            "SELECT agency_id, agency_name FROM collection_agencies_view ORDER BY agency_name ASC",
        )
        .load::<Row>(self.conn)?;

        Ok(rows
            .into_iter()
            .map(|r| Agency {
                id: r.agency_id,
                name: r.agency_name,
                country: String::new(),
                image_url: None,
                is_deleted: 0,
            })
            .collect())
    }
}

impl<'a> Repository<Agency> for AgencyRepository<'a> {
    fn insert(&mut self, item: Agency) -> RepoResult<Agency> {
        use crate::infrastructure::db::schema::agencies::dsl::*;

        diesel::insert_into(agencies)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Agency>> {
        use crate::infrastructure::db::schema::agencies::dsl::*;

        Ok(agencies
            .filter(id.eq(record_id))
            .first::<Agency>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Agency>> {
        use crate::infrastructure::db::schema::agencies::dsl::*;

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
        use crate::infrastructure::db::schema::agencies::dsl::*;

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
        use crate::infrastructure::db::schema::agencies::dsl::*;

        diesel::update(agencies.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
