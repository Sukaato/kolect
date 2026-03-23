use diesel::SqliteConnection;

use crate::db::repositories::{AgencyRepository, RepositoryError};
use crate::dto::output::AgencyDto;

pub struct AgencyService<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> AgencyService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns all non-deleted agencies (used on the Home / dataset screen).
    pub fn get_all(&mut self) -> Result<Vec<AgencyDto>, RepositoryError> {
        let mut repo = AgencyRepository::new(self.conn);
        let agencies = repo.find_all_active()?;
        Ok(agencies.into_iter().map(AgencyDto::from).collect())
    }

    /// Returns only agencies that have at least one item in the user collection.
    pub fn get_for_collection(&mut self) -> Result<Vec<AgencyDto>, RepositoryError> {
        let mut repo = AgencyRepository::new(self.conn);
        let agencies = repo.find_for_collection()?;
        Ok(agencies.into_iter().map(AgencyDto::from).collect())
    }
}
