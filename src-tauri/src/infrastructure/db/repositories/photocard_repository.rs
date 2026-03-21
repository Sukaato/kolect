use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Photocard;

// ─── Row enrichi ─────────────────────────────────────────────────────────────

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

    /// Retourne toutes les photocards liées à un album (via versions, digipacks
    /// ou rattachement direct) avec owned_count et has_signed.
    pub fn find_by_album_id_with_owned(
        &mut self,
        a_id: &str,
    ) -> RepoResult<Vec<PhotocardWithOwnedRow>> {
        let sql = "
            SELECT
                p.id,
                p.artist_id,
                p.album_version_id,
                p.digipack_id,
                p.image_url,
                COUNT(uc.id) AS owned_count,
                CASE WHEN SUM(uc.is_signed) > 0 THEN 1 ELSE 0 END AS has_signed
            FROM photocards p
            LEFT JOIN user_collection uc ON uc.photocard_id = p.id
            WHERE p.is_deleted = 0
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
        ";

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(a_id)
            .bind::<Text, _>(a_id)
            .bind::<Text, _>(a_id)
            .load::<PhotocardWithOwnedRow>(self.conn)?)
    }
}

impl<'a> Repository<Photocard> for PhotocardRepository<'a> {
    fn insert(&mut self, item: Photocard) -> RepoResult<Photocard> {
        use crate::infrastructure::db::schema::photocards::dsl::*;

        diesel::insert_into(photocards)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Photocard>> {
        use crate::infrastructure::db::schema::photocards::dsl::*;

        Ok(photocards
            .filter(id.eq(record_id))
            .first::<Photocard>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Photocard>> {
        use crate::infrastructure::db::schema::photocards::dsl::*;

        let total = photocards
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = photocards
            .filter(is_deleted.eq(0))
            .order(release_date.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Photocard>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Photocard) -> RepoResult<Photocard> {
        use crate::infrastructure::db::schema::photocards::dsl::*;

        diesel::update(photocards.filter(id.eq(&item.id)))
            .set((
                artist_id.eq(&item.artist_id),
                album_id.eq(&item.album_id),
                album_version_id.eq(&item.album_version_id),
                digipack_id.eq(&item.digipack_id),
                release_date.eq(&item.release_date),
                region.eq(&item.region),
                image_url.eq(&item.image_url),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::photocards::dsl::*;

        diesel::update(photocards.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
