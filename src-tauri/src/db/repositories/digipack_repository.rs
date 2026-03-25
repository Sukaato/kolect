use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::RepoResult;

// ─── Row ──────────────────────────────────────────────────────────────────────

#[derive(QueryableByName, Debug)]
pub struct DigipackWithOwnedRow {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub artist_id: Option<String>,
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

pub struct DigipackRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> DigipackRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns digipacks for an album with owned_count and has_signed.
    /// - include_exclusive_items : if false, only GLOBAL region digipacks are returned
    pub fn find_by_album_id_with_owned(
        &mut self,
        a_id: &str,
        include_exclusive_items: bool,
    ) -> RepoResult<Vec<DigipackWithOwnedRow>> {
        let region_filter = if include_exclusive_items {
            ""
        } else {
            "AND d.region = 'GLOBAL'"
        };

        let sql = format!(
            "
            SELECT
                d.id,
                d.name,
                d.artist_id,
                d.release_date,
                d.region,
                d.image_url,
                COUNT(uc.id) AS owned_count,
                CASE WHEN SUM(uc.is_signed) > 0 THEN 1 ELSE 0 END AS has_signed
            FROM digipacks d
            LEFT JOIN user_collection uc ON uc.digipack_id = d.id
            WHERE d.album_id = ? AND d.is_deleted = 0 {region_filter}
            GROUP BY d.id
            ORDER BY d.release_date ASC
        "
        );

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(a_id)
            .load::<DigipackWithOwnedRow>(self.conn)?)
    }
}
