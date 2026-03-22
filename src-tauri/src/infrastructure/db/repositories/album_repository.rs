use diesel::prelude::*;
use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Album;

// ─── Row enrichi ─────────────────────────────────────────────────────────────

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

    /// Retourne les albums d'un groupe avec comptage owned/total des versions.
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

    /// Retourne les albums d'un artiste solo avec comptage owned/total.
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

    /// Retourne le détail d'un album avec sa progression globale (versions uniquement).
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

impl<'a> Repository<Album> for AlbumRepository<'a> {
    fn insert(&mut self, item: Album) -> RepoResult<Album> {
        use crate::infrastructure::db::schema::albums::dsl::*;

        diesel::insert_into(albums)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Album>> {
        use crate::infrastructure::db::schema::albums::dsl::*;

        Ok(albums
            .filter(id.eq(record_id))
            .first::<Album>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Album>> {
        use crate::infrastructure::db::schema::albums::dsl::*;

        let total = albums
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = albums
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Album>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Album) -> RepoResult<Album> {
        use crate::infrastructure::db::schema::albums::dsl::*;

        diesel::update(albums.filter(id.eq(&item.id)))
            .set((
                name.eq(&item.name),
                release_date.eq(&item.release_date),
                region.eq(&item.region),
                image_url.eq(&item.image_url),
                group_id.eq(&item.group_id),
                artist_id.eq(&item.artist_id),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::albums::dsl::*;

        diesel::update(albums.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
