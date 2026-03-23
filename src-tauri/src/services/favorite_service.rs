use diesel::sqlite::SqliteConnection;

use crate::db::repositories::{RepositoryError, UserFavoriteRepository};

pub struct FavoriteService<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> FavoriteService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Toggle le favori d'un groupe.
    /// Retourne le nouvel état : true = favori, false = non favori.
    pub fn toggle_group(&mut self, group_id: &str) -> Result<bool, RepositoryError> {
        UserFavoriteRepository::new(self.conn).toggle_group_favorite(group_id)
    }

    /// Toggle le favori d'un artiste.
    /// Retourne le nouvel état : true = favori, false = non favori.
    pub fn toggle_artist(&mut self, artist_id: &str) -> Result<bool, RepositoryError> {
        UserFavoriteRepository::new(self.conn).toggle_artist_favorite(artist_id)
    }
}
