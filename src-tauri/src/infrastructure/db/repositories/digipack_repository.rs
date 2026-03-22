use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Digipack;

// ─── Row enrichi ─────────────────────────────────────────────────────────────

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

    /// Retourne les digipacks d'un album avec owned_count et has_signed.
    pub fn find_by_album_id_with_owned(
        &mut self,
        a_id: &str,
    ) -> RepoResult<Vec<DigipackWithOwnedRow>> {
        let sql = "
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
            WHERE d.album_id = ? AND d.is_deleted = 0
            GROUP BY d.id
            ORDER BY d.release_date ASC
        ";

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(a_id)
            .load::<DigipackWithOwnedRow>(self.conn)?)
    }
}

impl<'a> Repository<Digipack> for DigipackRepository<'a> {
    fn insert(&mut self, item: Digipack) -> RepoResult<Digipack> {
        use crate::infrastructure::db::schema::digipacks::dsl::*;

        diesel::insert_into(digipacks)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Digipack>> {
        use crate::infrastructure::db::schema::digipacks::dsl::*;

        Ok(digipacks
            .filter(id.eq(record_id))
            .first::<Digipack>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Digipack>> {
        use crate::infrastructure::db::schema::digipacks::dsl::*;

        let total = digipacks
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = digipacks
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Digipack>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Digipack) -> RepoResult<Digipack> {
        use crate::infrastructure::db::schema::digipacks::dsl::*;

        diesel::update(digipacks.filter(id.eq(&item.id)))
            .set((
                album_id.eq(&item.album_id),
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
        use crate::infrastructure::db::schema::digipacks::dsl::*;

        diesel::update(digipacks.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
