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
    #[diesel(sql_type = BigInt)]
    pub owned_count: i64,
    #[diesel(sql_type = BigInt)]
    pub total_count: i64,
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
    #[diesel(sql_type = BigInt)]
    pub owned_count: i64,
    #[diesel(sql_type = BigInt)]
    pub total_count: i64,
}

// ─── Repository ───────────────────────────────────────────────────────────────

pub struct AlbumRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> AlbumRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns albums for a group with owned/total version counts.
    pub fn find_summaries_by_group_id(&mut self, g_id: &str) -> RepoResult<Vec<AlbumSummaryRow>> {
        let sql = "
            SELECT
                al.id AS album_id,
                al.name,
                al.release_date,
                al.image_url,
                COUNT(DISTINCT uc.album_version_id) AS owned_count,
                COUNT(DISTINCT av.id) AS total_count
            FROM albums al
            LEFT JOIN album_versions av ON av.album_id = al.id AND av.is_deleted = 0
            LEFT JOIN user_collection uc ON uc.album_version_id = av.id
            WHERE al.group_id = ? AND al.is_deleted = 0
            GROUP BY al.id
            ORDER BY al.release_date DESC
        ";

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(g_id)
            .load::<AlbumSummaryRow>(self.conn)?)
    }

    /// Returns albums for a solo artist with owned/total version counts.
    pub fn find_summaries_by_artist_id(&mut self, a_id: &str) -> RepoResult<Vec<AlbumSummaryRow>> {
        let sql = "
            SELECT
                al.id AS album_id,
                al.name,
                al.release_date,
                al.image_url,
                COUNT(DISTINCT uc.album_version_id) AS owned_count,
                COUNT(DISTINCT av.id) AS total_count
            FROM albums al
            LEFT JOIN album_versions av ON av.album_id = al.id AND av.is_deleted = 0
            LEFT JOIN user_collection uc ON uc.album_version_id = av.id
            WHERE al.artist_id = ? AND al.is_deleted = 0
            GROUP BY al.id
            ORDER BY al.release_date DESC
        ";

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(a_id)
            .load::<AlbumSummaryRow>(self.conn)?)
    }

    /// Returns the detail of an album with its overall progress (versions only).
    pub fn find_detail_by_id(&mut self, record_id: &str) -> RepoResult<Option<AlbumDetailRow>> {
        let sql = "
            SELECT
                al.id AS album_id,
                al.name,
                al.release_date,
                al.image_url,
                al.group_id,
                al.artist_id,
                COUNT(DISTINCT uc.album_version_id) AS owned_count,
                COUNT(DISTINCT av.id) AS total_count
            FROM albums al
            LEFT JOIN album_versions av ON av.album_id = al.id AND av.is_deleted = 0
            LEFT JOIN user_collection uc ON uc.album_version_id = av.id
            WHERE al.id = ? AND al.is_deleted = 0
            GROUP BY al.id
        ";

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(record_id)
            .get_result::<AlbumDetailRow>(self.conn)
            .optional()?)
    }
}
