use diesel::prelude::*;
use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::RepoResult;

// ─── Row ──────────────────────────────────────────────────────────────────────

#[derive(QueryableByName, Debug)]
pub struct LightstickWithOwnedRow {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub group_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub artist_id: Option<String>,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub version: String,
    #[diesel(sql_type = Text)]
    pub release_date: String,
    #[diesel(sql_type = Text)]
    pub region: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub image_url: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub owned_count: i64,
}

// ─── Repository ───────────────────────────────────────────────────────────────

pub struct LightstickRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> LightstickRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns lightsticks for a group with owned_count.
    /// - include_exclusive_items : if false, only GLOBAL region lightsticks are returned
    pub fn find_by_group_id_with_owned(
        &mut self,
        g_id: &str,
        include_exclusive_items: bool,
    ) -> RepoResult<Vec<LightstickWithOwnedRow>> {
        let region_filter = if include_exclusive_items {
            ""
        } else {
            "AND ls.region = 'GLOBAL'"
        };

        let sql = format!(
            "
            SELECT
                ls.id,
                ls.group_id,
                ls.artist_id,
                ls.name,
                ls.version,
                ls.release_date,
                ls.region,
                ls.image_url,
                COUNT(uc.lightstick_id) AS owned_count
            FROM lightsticks ls
            LEFT JOIN user_collection uc ON uc.lightstick_id = ls.id
            WHERE ls.group_id = ? AND ls.is_deleted = 0 {region_filter}
            GROUP BY ls.id
            ORDER BY ls.release_date ASC
        "
        );

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(g_id)
            .load::<LightstickWithOwnedRow>(self.conn)?)
    }

    /// Returns lightsticks for a solo artist with owned_count.
    /// - include_exclusive_items : if false, only GLOBAL region lightsticks are returned
    pub fn find_by_artist_id_with_owned(
        &mut self,
        a_id: &str,
        include_exclusive_items: bool,
    ) -> RepoResult<Vec<LightstickWithOwnedRow>> {
        let region_filter = if include_exclusive_items {
            ""
        } else {
            "AND ls.region = 'GLOBAL'"
        };

        let sql = format!(
            "
            SELECT
                ls.id,
                ls.group_id,
                ls.artist_id,
                ls.name,
                ls.version,
                ls.release_date,
                ls.region,
                ls.image_url,
                COUNT(uc.lightstick_id) AS owned_count
            FROM lightsticks ls
            LEFT JOIN user_collection uc ON uc.lightstick_id = ls.id
            WHERE ls.artist_id = ? AND ls.is_deleted = 0 {region_filter}
            GROUP BY ls.id
            ORDER BY ls.release_date ASC
        "
        );

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(a_id)
            .load::<LightstickWithOwnedRow>(self.conn)?)
    }
}
