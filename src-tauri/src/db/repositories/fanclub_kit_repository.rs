use diesel::prelude::*;
use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::RepoResult;

// ─── Row ──────────────────────────────────────────────────────────────────────

#[derive(QueryableByName, Debug)]
pub struct FanclubKitWithOwnedRow {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub group_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub artist_id: Option<String>,
    #[diesel(sql_type = Text)]
    pub name: String,
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

pub struct FanclubKitRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> FanclubKitRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns fanclub kits for a group with owned_count.
    pub fn find_by_group_id_with_owned(
        &mut self,
        g_id: &str,
    ) -> RepoResult<Vec<FanclubKitWithOwnedRow>> {
        let sql = "
            SELECT
                fk.id,
                fk.group_id,
                fk.artist_id,
                fk.name,
                fk.release_date,
                fk.region,
                fk.image_url,
                COUNT(uc.fanclub_kit_id) AS owned_count
            FROM fanclub_kits fk
            LEFT JOIN user_collection uc ON uc.fanclub_kit_id = fk.id
            WHERE fk.group_id = ? AND fk.is_deleted = 0
            GROUP BY fk.id
            ORDER BY fk.release_date ASC
        ";

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(g_id)
            .load::<FanclubKitWithOwnedRow>(self.conn)?)
    }

    /// Returns fanclub kits for a solo artist with owned_count.
    pub fn find_by_artist_id_with_owned(
        &mut self,
        a_id: &str,
    ) -> RepoResult<Vec<FanclubKitWithOwnedRow>> {
        let sql = "
            SELECT
                fk.id,
                fk.group_id,
                fk.artist_id,
                fk.name,
                fk.release_date,
                fk.region,
                fk.image_url,
                COUNT(uc.fanclub_kit_id) AS owned_count
            FROM fanclub_kits fk
            LEFT JOIN user_collection uc ON uc.fanclub_kit_id = fk.id
            WHERE fk.artist_id = ? AND fk.is_deleted = 0
            GROUP BY fk.id
            ORDER BY fk.release_date ASC
        ";

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(a_id)
            .load::<FanclubKitWithOwnedRow>(self.conn)?)
    }
}
