use std::borrow::Cow;

use diesel::prelude::*;
use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::RepoResult;

// ─── Rows ─────────────────────────────────────────────────────────────────────

#[derive(QueryableByName, Debug)]
pub struct AlbumSummaryRow {
    #[diesel(sql_type = Text)]
    pub album_id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub release_date: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub image_url: Option<String>,
    // Versions
    #[diesel(sql_type = BigInt)]
    pub versions_owned_count: i64,
    #[diesel(sql_type = BigInt)]
    pub versions_total_count: i64,
    // Digipacks
    #[diesel(sql_type = BigInt)]
    pub digipacks_owned_count: i64,
    #[diesel(sql_type = BigInt)]
    pub digipacks_total_count: i64,
    // Photocards
    #[diesel(sql_type = BigInt)]
    pub photocards_owned_count: i64,
}

#[derive(QueryableByName, Debug)]
pub struct AlbumDetailRow {
    #[diesel(sql_type = Text)]
    pub album_id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub release_date: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub image_url: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub group_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub artist_id: Option<String>,
    // Versions
    #[diesel(sql_type = BigInt)]
    pub versions_owned_count: i64,
    #[diesel(sql_type = BigInt)]
    pub versions_total_count: i64,
    // Digipacks
    #[diesel(sql_type = BigInt)]
    pub digipacks_owned_count: i64,
    #[diesel(sql_type = BigInt)]
    pub digipacks_total_count: i64,
    // Photocards
    #[diesel(sql_type = BigInt)]
    pub photocards_owned_count: i64,
    #[diesel(sql_type = BigInt)]
    pub photocards_total_count: i64,
}

// ─── Repository ───────────────────────────────────────────────────────────────

pub struct AlbumRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> AlbumRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns albums for a group with owned/total counts for versions, digipacks and photocards.
    /// - include_exclusive_items : if false, only GLOBAL region items are counted
    pub fn find_summaries_by_group_id(
        &mut self,
        g_id: &str,
        include_exclusive_items: bool,
    ) -> RepoResult<Vec<AlbumSummaryRow>> {
        let album_version_region = region_clause(include_exclusive_items, "av").into_owned();
        let digipack_region = region_clause(include_exclusive_items, "d").into_owned();
        let photocard_region = region_clause(include_exclusive_items, "p").into_owned();

        let sql = format!("
            SELECT
                al.id AS album_id,
                al.name,
                al.release_date,
                al.image_url,

                COUNT(DISTINCT av.id)                  AS versions_total_count,
                COUNT(DISTINCT uc_av.album_version_id) AS versions_owned_count,

                COUNT(DISTINCT d.id)                   AS digipacks_total_count,
                COUNT(DISTINCT uc_d.digipack_id)       AS digipacks_owned_count,

                COUNT(DISTINCT uc_p.photocard_id)      AS photocards_owned_count

            FROM albums al
            LEFT JOIN album_versions av ON av.album_id = al.id AND av.is_deleted = 0 {album_version_region}
            LEFT JOIN user_collection uc_av ON uc_av.album_version_id = av.id
            LEFT JOIN digipacks d ON d.album_id = al.id AND d.is_deleted = 0 {digipack_region}
            LEFT JOIN user_collection uc_d ON uc_d.digipack_id = d.id
            LEFT JOIN photocards p ON p.is_deleted = 0 {photocard_region} AND (
                p.album_version_id IN (
                    SELECT id FROM album_versions WHERE album_id = al.id AND is_deleted = 0 {album_version_region}
                )
                OR p.digipack_id IN (
                    SELECT id FROM digipacks WHERE album_id = al.id AND is_deleted = 0 {digipack_region}
                )
                OR p.album_id = al.id
            )
            LEFT JOIN user_collection uc_p ON uc_p.photocard_id = p.id
            WHERE al.group_id = ? AND al.is_deleted = 0
            GROUP BY al.id
            ORDER BY al.release_date DESC
        ");

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(g_id)
            .load::<AlbumSummaryRow>(self.conn)?)
    }

    /// Returns albums for a solo artist with owned/total counts for versions, digipacks and photocards.
    /// - include_exclusive_items : if false, only GLOBAL region items are counted
    pub fn find_summaries_by_artist_id(
        &mut self,
        a_id: &str,
        include_exclusive_items: bool,
    ) -> RepoResult<Vec<AlbumSummaryRow>> {
        let album_version_region = region_clause(include_exclusive_items, "av").into_owned();
        let digipack_region = region_clause(include_exclusive_items, "d").into_owned();
        let photocard_region = region_clause(include_exclusive_items, "p").into_owned();

        let sql = format!("
            SELECT
                al.id AS album_id,
                al.name,
                al.release_date,
                al.image_url,

                COUNT(DISTINCT av.id)                  AS versions_total_count,
                COUNT(DISTINCT uc_av.album_version_id) AS versions_owned_count,

                COUNT(DISTINCT d.id)                   AS digipacks_total_count,
                COUNT(DISTINCT uc_d.digipack_id)       AS digipacks_owned_count,

                COUNT(DISTINCT uc_p.photocard_id)      AS photocards_owned_count

            FROM albums al
            LEFT JOIN album_versions av ON av.album_id = al.id AND av.is_deleted = 0 {album_version_region}
            LEFT JOIN user_collection uc_av ON uc_av.album_version_id = av.id
            LEFT JOIN digipacks d ON d.album_id = al.id AND d.is_deleted = 0 {digipack_region}
            LEFT JOIN user_collection uc_d ON uc_d.digipack_id = d.id
            LEFT JOIN photocards p ON p.is_deleted = 0 {photocard_region} AND (
                p.album_version_id IN (
                    SELECT id FROM album_versions WHERE album_id = al.id AND is_deleted = 0 {digipack_region}
                )
                OR p.digipack_id IN (
                    SELECT id FROM digipacks WHERE album_id = al.id AND is_deleted = 0 {digipack_region}
                )
                OR p.album_id = al.id
            )
            LEFT JOIN user_collection uc_p ON uc_p.photocard_id = p.id
            WHERE al.artist_id = ? AND al.is_deleted = 0
            GROUP BY al.id
            ORDER BY al.release_date DESC
        ");

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(a_id)
            .load::<AlbumSummaryRow>(self.conn)?)
    }

    /// Returns the detail of an album with separate owned/total counts
    /// for versions, digipacks and photocards.
    /// - include_exclusive_items : if false, only GLOBAL region items are counted
    pub fn find_detail_by_id(
        &mut self,
        record_id: &str,
        include_exclusive_items: bool,
    ) -> RepoResult<Option<AlbumDetailRow>> {
        let album_version_region = region_clause(include_exclusive_items, "av").into_owned();
        let digipack_region = region_clause(include_exclusive_items, "d").into_owned();
        let photocard_region = region_clause(include_exclusive_items, "p").into_owned();

        let sql = format!(
            "
            SELECT
                al.id AS album_id,
                al.name,
                al.release_date,
                al.image_url,
                al.group_id,
                al.artist_id,

                COUNT(DISTINCT av.id)              AS versions_total_count,
                COUNT(DISTINCT uc_av.album_version_id) AS versions_owned_count,

                COUNT(DISTINCT d.id)               AS digipacks_total_count,
                COUNT(DISTINCT uc_d.digipack_id)   AS digipacks_owned_count,

                COUNT(DISTINCT p.id)               AS photocards_total_count,
                COUNT(DISTINCT uc_p.photocard_id)  AS photocards_owned_count

            FROM albums al

            LEFT JOIN album_versions av
                ON av.album_id = al.id AND av.is_deleted = 0 {album_version_region}
            LEFT JOIN user_collection uc_av
                ON uc_av.album_version_id = av.id

            LEFT JOIN digipacks d
                ON d.album_id = al.id AND d.is_deleted = 0 {digipack_region}
            LEFT JOIN user_collection uc_d
                ON uc_d.digipack_id = d.id

            LEFT JOIN photocards p
                ON p.is_deleted = 0 {photocard_region} AND (
                    p.album_version_id IN (
                        SELECT id FROM album_versions
                        WHERE album_id = al.id AND is_deleted = 0 {digipack_region}
                    )
                    OR p.digipack_id IN (
                        SELECT id FROM digipacks
                        WHERE album_id = al.id AND is_deleted = 0 {digipack_region}
                    )
                    OR p.album_id = al.id
                )
            LEFT JOIN user_collection uc_p
                ON uc_p.photocard_id = p.id

            WHERE al.id = ? AND al.is_deleted = 0
            GROUP BY al.id
        "
        );

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(record_id)
            .get_result::<AlbumDetailRow>(self.conn)
            .optional()?)
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Returns an extra AND clause to restrict items to GLOBAL region when exclusive items are excluded.
fn region_clause(include_exclusive_items: bool, table: &'static str) -> Cow<'static, str> {
    if include_exclusive_items {
        Cow::Borrowed("")
    } else {
        Cow::Owned(format!("AND {}.region = 'GLOBAL'", table))
    }
}
