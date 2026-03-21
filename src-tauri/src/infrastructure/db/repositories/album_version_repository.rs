use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::AlbumVersion;

// ─── Row enrichi ─────────────────────────────────────────────────────────────

#[derive(QueryableByName, Debug)]
pub struct AlbumVersionWithOwnedRow {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub format: String,
    #[diesel(sql_type = Text)]
    pub release_date: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub image_url: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub owned_count: i64,
    #[diesel(sql_type = Bool)]
    pub has_signed: bool,
}

// ─── Repository ───────────────────────────────────────────────────────────────

pub struct AlbumVersionRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> AlbumVersionRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Retourne les versions d'un album avec owned_count et has_signed.
    pub fn find_by_album_id_with_owned(
        &mut self,
        a_id: &str,
    ) -> RepoResult<Vec<AlbumVersionWithOwnedRow>> {
        let sql = "
            SELECT
                av.id,
                av.name,
                av.format,
                av.release_date,
                av.image_url,
                COUNT(uc.id) AS owned_count,
                CASE WHEN SUM(uc.is_signed) > 0 THEN 1 ELSE 0 END AS has_signed
            FROM album_versions av
            LEFT JOIN user_collection uc ON uc.album_version_id = av.id
            WHERE av.album_id = ? AND av.is_deleted = 0
            GROUP BY av.id
            ORDER BY av.release_date ASC
        ";

        Ok(diesel::sql_query(sql)
            .bind::<Text, _>(a_id)
            .load::<AlbumVersionWithOwnedRow>(self.conn)?)
    }
}

impl<'a> Repository<AlbumVersion> for AlbumVersionRepository<'a> {
    fn insert(&mut self, item: AlbumVersion) -> RepoResult<AlbumVersion> {
        use crate::infrastructure::db::schema::album_versions::dsl::*;

        diesel::insert_into(album_versions)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<AlbumVersion>> {
        use crate::infrastructure::db::schema::album_versions::dsl::*;

        Ok(album_versions
            .filter(id.eq(record_id))
            .first::<AlbumVersion>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<AlbumVersion>> {
        use crate::infrastructure::db::schema::album_versions::dsl::*;

        let total = album_versions
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = album_versions
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<AlbumVersion>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: AlbumVersion) -> RepoResult<AlbumVersion> {
        use crate::infrastructure::db::schema::album_versions::dsl::*;

        diesel::update(album_versions.filter(id.eq(&item.id)))
            .set((
                album_id.eq(&item.album_id),
                name.eq(&item.name),
                format.eq(&item.format),
                release_date.eq(&item.release_date),
                region.eq(&item.region),
                image_url.eq(&item.image_url),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::album_versions::dsl::*;

        diesel::update(album_versions.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}