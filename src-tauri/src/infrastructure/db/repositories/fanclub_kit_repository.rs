use diesel::prelude::*;
use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::FanclubKit;

// ─── Row enrichi ─────────────────────────────────────────────────────────────

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

    /// Retourne les fanclub kits d'un groupe avec owned_count.
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

    /// Retourne les fanclub kits d'un artiste solo avec owned_count.
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

impl<'a> Repository<FanclubKit> for FanclubKitRepository<'a> {
    fn insert(&mut self, item: FanclubKit) -> RepoResult<FanclubKit> {
        use crate::infrastructure::db::schema::fanclub_kits::dsl::*;

        diesel::insert_into(fanclub_kits)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<FanclubKit>> {
        use crate::infrastructure::db::schema::fanclub_kits::dsl::*;

        Ok(fanclub_kits
            .filter(id.eq(record_id))
            .first::<FanclubKit>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<FanclubKit>> {
        use crate::infrastructure::db::schema::fanclub_kits::dsl::*;

        let total = fanclub_kits
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = fanclub_kits
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<FanclubKit>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: FanclubKit) -> RepoResult<FanclubKit> {
        use crate::infrastructure::db::schema::fanclub_kits::dsl::*;

        diesel::update(fanclub_kits.filter(id.eq(&item.id)))
            .set((
                group_id.eq(&item.group_id),
                artist_id.eq(&item.artist_id),
                name.eq(&item.name),
                release_date.eq(&item.release_date),
                region.eq(&item.region),
                image_url.eq(&item.image_url),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::fanclub_kits::dsl::*;

        diesel::update(fanclub_kits.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
