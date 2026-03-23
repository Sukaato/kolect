use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::RepoResult;
use crate::db::models::Agency;

pub struct AgencyRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> AgencyRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns all non-deleted agencies, ordered by name.
    pub fn find_all_active(&mut self) -> RepoResult<Vec<Agency>> {
        use crate::db::schema::agencies::dsl::*;

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
