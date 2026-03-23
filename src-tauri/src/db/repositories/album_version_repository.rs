use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::RepoResult;

// ─── Row ──────────────────────────────────────────────────────────────────────

#[derive(QueryableByName, Debug)]
pub struct AlbumVersionWithOwnedRow {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub format: String,
    #[diesel(sql_type = Text)]
    pub release_date: String,
    #[diesel(sql_type = Text)]
    pub region: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub image_url: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub owned_count: i64,
    #[diesel(sql_type = Bool)]
    pub has_signed: bool,
}

// ─── Repository ───────────────────────────────────────────────────────────────

pub struct AlbumVersionRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> AlbumVersionRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns versions of an album with owned_count and has_signed.
    pub fn find_by_album_id_with_owned(
        &mut self,
        a_id: &str,
    ) -> RepoResult<Vec<AlbumVersionWithOwnedRow>> {
        let sql = "
            SELECT
                av.id,
                av.name,
                av.format,
                av.release_date,
                av.region,
                av.image_url,
                COUNT(uc.id) AS owned_count,
                CASE WHEN SUM(uc.is_signed) > 0 THEN 1 ELSE 0 END AS has_signed
            FROM album_versions av
            LEFT JOIN user_collection uc ON uc.album_version_id = av.id
            WHERE av.album_id = ? AND av.is_deleted = 0
            GROUP BY av.id
            ORDER BY av.release_date ASC
        ";

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(a_id)
            .load::<AlbumVersionWithOwnedRow>(self.conn)?)
    }
}
