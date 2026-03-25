use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::RepoResult;

// ─── Row ──────────────────────────────────────────────────────────────────────

#[derive(QueryableByName, Debug)]
pub struct PhotocardWithOwnedRow {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub artist_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub album_version_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub digipack_id: Option<String>,
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

pub struct PhotocardRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> PhotocardRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns all photocards linked to an album with owned_count and has_signed.
    /// - include_exclusive_items : if false, only GLOBAL region photocards are returned
    pub fn find_by_album_id_with_owned(
        &mut self,
        a_id: &str,
        include_exclusive_items: bool,
    ) -> RepoResult<Vec<PhotocardWithOwnedRow>> {
        let region_filter = if include_exclusive_items {
            ""
        } else {
            "AND p.region = 'GLOBAL'"
        };

        let sql = format!(
            "
            SELECT
                p.id,
                p.artist_id,
                p.album_version_id,
                p.digipack_id,
                p.region,
                p.image_url,
                COUNT(uc.id) AS owned_count,
                CASE WHEN SUM(uc.is_signed) > 0 THEN 1 ELSE 0 END AS has_signed
            FROM photocards p
            LEFT JOIN user_collection uc ON uc.photocard_id = p.id
            WHERE p.is_deleted = 0 {region_filter}
              AND (
                p.album_version_id IN (
                    SELECT id FROM album_versions WHERE album_id = ? AND is_deleted = 0
                )
                OR p.digipack_id IN (
                    SELECT id FROM digipacks WHERE album_id = ? AND is_deleted = 0
                )
                OR p.album_id = ?
              )
            GROUP BY p.id
            ORDER BY p.artist_id ASC
        "
        );

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(a_id)
            .bind::<Text, _>(a_id)
            .bind::<Text, _>(a_id)
            .load::<PhotocardWithOwnedRow>(self.conn)?)
    }
}
