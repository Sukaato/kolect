// src-tauri/src/infrastructure/db/repositories/group_repository.rs

use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult, Repository, RepositoryError};
use crate::infrastructure::db::models::Group;

// ─── Row interne pour get_summary ────────────────────────────────────────────

#[derive(QueryableByName, Debug)]
pub struct GroupSummaryRow {
    #[diesel(sql_type = Text)]
    pub id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub agency_name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub image_url: Option<String>,
    #[diesel(sql_type = Bool)]
    pub is_favorite: bool,
    #[diesel(sql_type = BigInt)]
    pub total_count: i64,
    #[diesel(sql_type = BigInt)]
    pub owned_count: i64,
}

// ─── Repository ───────────────────────────────────────────────────────────────

pub struct GroupRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> GroupRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Retourne le sommaire des groupes avec comptage owned/total.
    /// - owned_only  : true → HAVING owned_count > 0 (collection), false → tous (home)
    /// - query       : recherche FTS sur le nom
    /// - agency_ids  : filtre par agences
    /// - include_photocards : inclure les photocards dans le comptage
    pub fn get_summary(
        &mut self,
        owned_only: bool,
        query: Option<&str>,
        agency_ids: Option<&[String]>,
        include_photocards: bool,
    ) -> Result<Vec<GroupSummaryRow>, diesel::result::Error> {
        let photocard_union = if include_photocards {
            "UNION ALL
             SELECT p.id AS item_id, al.group_id AS owner_id
             FROM photocards p
             JOIN album_versions av ON av.id = p.album_version_id AND av.is_deleted = 0
             JOIN albums al ON al.id = av.album_id AND al.group_id IS NOT NULL AND al.is_deleted = 0
             WHERE p.is_deleted = 0
             UNION ALL
             SELECT p.id AS item_id, al.group_id AS owner_id
             FROM photocards p
             JOIN digipacks d ON d.id = p.digipack_id AND d.is_deleted = 0
             JOIN albums al ON al.id = d.album_id AND al.group_id IS NOT NULL AND al.is_deleted = 0
             WHERE p.is_deleted = 0"
        } else {
            ""
        };

        let owned_photocard_union = if include_photocards {
            "UNION ALL
             SELECT uc.photocard_id AS item_id, al.group_id AS owner_id
             FROM user_collection uc
             JOIN photocards p ON p.id = uc.photocard_id
             JOIN album_versions av ON av.id = p.album_version_id
             JOIN albums al ON al.id = av.album_id AND al.group_id IS NOT NULL
             WHERE uc.photocard_id IS NOT NULL
             UNION ALL
             SELECT uc.photocard_id AS item_id, al.group_id AS owner_id
             FROM user_collection uc
             JOIN photocards p ON p.id = uc.photocard_id
             JOIN digipacks d ON d.id = p.digipack_id
             JOIN albums al ON al.id = d.album_id AND al.group_id IS NOT NULL
             WHERE uc.photocard_id IS NOT NULL"
        } else {
            ""
        };

        // Filtre FTS
        let fts_join = if query.is_some() {
            "JOIN collection_groups_fts fts ON fts.group_id = g.id"
        } else {
            ""
        };

        let fts_where = if let Some(q) = query {
            format!("AND fts.name MATCH '{q}*'")
        } else {
            String::new()
        };

        // Filtre agence
        let agency_filter = if let Some(ids) = agency_ids.filter(|ids| !ids.is_empty()) {
            let placeholders = ids
                .iter()
                .map(|a_id| format!("'{a_id}'"))
                .collect::<Vec<_>>()
                .join(", ");
            format!("AND g.agency_id IN ({placeholders})")
        } else {
            String::new()
        };

        let having = if owned_only {
            "HAVING owned_count > 0"
        } else {
            ""
        };

        let sql = format!(
            "SELECT
                g.id,
                g.name,
                a.name AS agency_name,
                g.image_url,
                CASE WHEN ufg.group_id IS NOT NULL THEN 1 ELSE 0 END AS is_favorite,
                COUNT(DISTINCT total_items.item_id) AS total_count,
                COUNT(DISTINCT owned_items.item_id) AS owned_count
            FROM groups g
            JOIN agencies a ON a.id = g.agency_id
            {fts_join}
            LEFT JOIN user_favorites_groups ufg ON ufg.group_id = g.id
            LEFT JOIN (
                SELECT av.id AS item_id, al.group_id AS owner_id
                FROM album_versions av
                JOIN albums al ON al.id = av.album_id AND al.is_deleted = 0
                WHERE av.is_deleted = 0 AND al.group_id IS NOT NULL
                UNION ALL
                SELECT d.id AS item_id, al.group_id AS owner_id
                FROM digipacks d
                JOIN albums al ON al.id = d.album_id AND al.is_deleted = 0
                WHERE d.is_deleted = 0 AND al.group_id IS NOT NULL
                UNION ALL
                SELECT ls.id AS item_id, ls.group_id AS owner_id
                FROM lightsticks ls
                WHERE ls.is_deleted = 0 AND ls.group_id IS NOT NULL
                UNION ALL
                SELECT fk.id AS item_id, fk.group_id AS owner_id
                FROM fanclub_kits fk
                WHERE fk.is_deleted = 0 AND fk.group_id IS NOT NULL
                {photocard_union}
            ) total_items ON total_items.owner_id = g.id
            LEFT JOIN (
                SELECT uc.album_version_id AS item_id, al.group_id AS owner_id
                FROM user_collection uc
                JOIN album_versions av ON av.id = uc.album_version_id
                JOIN albums al ON al.id = av.album_id AND al.group_id IS NOT NULL
                WHERE uc.album_version_id IS NOT NULL
                UNION ALL
                SELECT uc.digipack_id AS item_id, al.group_id AS owner_id
                FROM user_collection uc
                JOIN digipacks d ON d.id = uc.digipack_id
                JOIN albums al ON al.id = d.album_id AND al.group_id IS NOT NULL
                WHERE uc.digipack_id IS NOT NULL
                UNION ALL
                SELECT uc.lightstick_id AS item_id, ls.group_id AS owner_id
                FROM user_collection uc
                JOIN lightsticks ls ON ls.id = uc.lightstick_id AND ls.group_id IS NOT NULL
                WHERE uc.lightstick_id IS NOT NULL
                UNION ALL
                SELECT uc.fanclub_kit_id AS item_id, fk.group_id AS owner_id
                FROM user_collection uc
                JOIN fanclub_kits fk ON fk.id = uc.fanclub_kit_id AND fk.group_id IS NOT NULL
                WHERE uc.fanclub_kit_id IS NOT NULL
                {owned_photocard_union}
            ) owned_items ON owned_items.owner_id = g.id
            WHERE g.is_deleted = 0
            {fts_where}
            {agency_filter}
            GROUP BY g.id
            {having}",
        );

        diesel::sql_query(sql).load::<GroupSummaryRow>(self.conn)
    }
}

impl<'a> Repository<Group> for GroupRepository<'a> {
    fn insert(&mut self, item: Group) -> RepoResult<Group> {
        use crate::infrastructure::db::schema::groups::dsl::*;

        diesel::insert_into(groups)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Group>> {
        use crate::infrastructure::db::schema::groups::dsl::*;

        Ok(groups
            .filter(id.eq(record_id))
            .first::<Group>(self.conn)
            .optional()?)
    }

    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<Group>> {
        use crate::infrastructure::db::schema::groups::dsl::*;

        let total = groups
            .filter(is_deleted.eq(0))
            .count()
            .get_result::<i64>(self.conn)?;

        let data = groups
            .filter(is_deleted.eq(0))
            .order(name.asc())
            .limit(page.limit())
            .offset(page.offset())
            .load::<Group>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    fn update(&mut self, item: Group) -> RepoResult<Group> {
        use crate::infrastructure::db::schema::groups::dsl::*;

        diesel::update(groups.filter(id.eq(&item.id)))
            .set((
                name.eq(&item.name),
                debut_date.eq(&item.debut_date),
                agency_id.eq(&item.agency_id),
                fandom_name.eq(&item.fandom_name),
                image_url.eq(&item.image_url),
            ))
            .execute(self.conn)?;
        self.find_by_id(&item.id)?
            .ok_or_else(|| RepositoryError::NotFound(item.id))
    }

    fn soft_delete(&mut self, record_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::groups::dsl::*;

        diesel::update(groups.filter(id.eq(record_id)))
            .set(is_deleted.eq(1))
            .execute(self.conn)?;
        Ok(())
    }
}
