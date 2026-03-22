use diesel::prelude::*;
use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Lightstick;

// ─── Row enrichi ─────────────────────────────────────────────────────────────

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

    pub fn find_by_group_id_with_owned(
        &mut self,
        g_id: &str,
    ) -> RepoResult<Vec<LightstickWithOwnedRow>> {
        let sql = "
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
            WHERE ls.group_id = ? AND ls.is_deleted = 0
            GROUP BY ls.id
            ORDER BY ls.release_date ASC
        ";

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(g_id)
            .load::<LightstickWithOwnedRow>(self.conn)?)
    }

    pub fn find_by_artist_id_with_owned(
        &mut self,
        a_id: &str,
    ) -> RepoResult<Vec<LightstickWithOwnedRow>> {
        let sql = "
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
            WHERE ls.artist_id = ? AND ls.is_deleted = 0
            GROUP BY ls.id
            ORDER BY ls.release_date ASC
        ";

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(a_id)
            .load::<LightstickWithOwnedRow>(self.conn)?)
    }
}

impl<'a> Repository<Lightstick> for LightstickRepository<'a> {
    fn insert(&mut self, item: Lightstick) -> RepoResult<Lightstick> {
        use crate::infrastructure::db::schema::lightsticks::dsl::*;

        diesel::insert_into(lightsticks)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Lightstick>> {
        use crate::infrastructure::db::schema::lightsticks::dsl::*;

        Ok(lightsticks
            .filter(id.eq(record_id))
            .first::<Lightstick>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Lightstick>> {
        use crate::infrastructure::db::schema::lightsticks::dsl::*;

        let total = lightsticks
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = lightsticks
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Lightstick>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Lightstick) -> RepoResult<Lightstick> {
        use crate::infrastructure::db::schema::lightsticks::dsl::*;

        diesel::update(lightsticks.filter(id.eq(&item.id)))
            .set((
                group_id.eq(&item.group_id),
                artist_id.eq(&item.artist_id),
                name.eq(&item.name),
                version.eq(&item.version),
                release_date.eq(&item.release_date),
                image_url.eq(&item.image_url),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::lightsticks::dsl::*;

        diesel::update(lightsticks.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
