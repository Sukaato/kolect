use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::RepoResult;

// ─── Row ──────────────────────────────────────────────────────────────────────

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

    /// Returns a single group by its ID.
    pub fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<crate::db::models::Group>> {
        use crate::db::schema::groups::dsl::*;

        Ok(groups
            .filter(id.eq(record_id))
            .filter(is_deleted.eq(0))
            .first::<crate::db::models::Group>(self.conn)
            .optional()?)
    }

    /// Returns the group summary with owned/total counts.
    /// - owned_only              : true → HAVING owned_count > 0 (collection screen)
    /// +                           false → all groups (home screen)
    /// - query                   : LIKE %query% search on group name (case-insensitive)
    /// - agency_ids              : filter by agency
    /// - include_photocards      : count photocards in total/owned
    /// - include_exclusive_items : if false, only GLOBAL region items are counted
    pub fn get_summary(
        &mut self,
        owned_only: bool,
        query: Option<&str>,
        agency_ids: Option<&[String]>,
        include_photocards: bool,
        include_exclusive_items: bool,
    ) -> Result<Vec<GroupSummaryRow>, diesel::result::Error> {
        let region_filter = if include_exclusive_items {
            ""
        } else {
            "AND av.region = 'GLOBAL'"
        };

        let region_filter_d = if include_exclusive_items {
            ""
        } else {
            "AND d.region = 'GLOBAL'"
        };

        let region_filter_ls = if include_exclusive_items {
            ""
        } else {
            "AND ls.region = 'GLOBAL'"
        };

        let region_filter_fk = if include_exclusive_items {
            ""
        } else {
            "AND fk.region = 'GLOBAL'"
        };

        let region_filter_p = if include_exclusive_items {
            ""
        } else {
            "AND p.region = 'GLOBAL'"
        };

        let photocard_union = if include_photocards {
            format!(
                "UNION ALL
                 SELECT p.id AS item_id, al.group_id AS owner_id
                 FROM photocards p
                 JOIN album_versions av ON av.id = p.album_version_id AND av.is_deleted = 0
                 JOIN albums al ON al.id = av.album_id AND al.group_id IS NOT NULL AND al.is_deleted = 0
                 WHERE p.is_deleted = 0 {region_filter_p}
                 UNION ALL
                 SELECT p.id AS item_id, al.group_id AS owner_id
                 FROM photocards p
                 JOIN digipacks d ON d.id = p.digipack_id AND d.is_deleted = 0
                 JOIN albums al ON al.id = d.album_id AND al.group_id IS NOT NULL AND al.is_deleted = 0
                 WHERE p.is_deleted = 0 {region_filter_p}"
            )
        } else {
            String::new()
        };

        let owned_photocard_union = if include_photocards {
            format!(
                "UNION ALL
                 SELECT uc.photocard_id AS item_id, al.group_id AS owner_id
                 FROM user_collection uc
                 JOIN photocards p ON p.id = uc.photocard_id {region_filter_p}
                 JOIN album_versions av ON av.id = p.album_version_id
                 JOIN albums al ON al.id = av.album_id AND al.group_id IS NOT NULL
                 WHERE uc.photocard_id IS NOT NULL
                 UNION ALL
                 SELECT uc.photocard_id AS item_id, al.group_id AS owner_id
                 FROM user_collection uc
                 JOIN photocards p ON p.id = uc.photocard_id {region_filter_p}
                 JOIN digipacks d ON d.id = p.digipack_id
                 JOIN albums al ON al.id = d.album_id AND al.group_id IS NOT NULL
                 WHERE uc.photocard_id IS NOT NULL"
            )
        } else {
            String::new()
        };

        let search_filter = match query {
            Some(q) => format!("AND g.name LIKE '%{}%'", q.replace('\'', "''")),
            None => String::new(),
        };

        let agency_filter = match agency_ids.filter(|ids| !ids.is_empty()) {
            Some(ids) => {
                let placeholders = ids
                    .iter()
                    .map(|a_id| format!("'{}'", a_id.replace('\'', "''")))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("AND g.agency_id IN ({placeholders})")
            }
            None => String::new(),
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
            LEFT JOIN user_favorites_groups ufg ON ufg.group_id = g.id
            LEFT JOIN (
                SELECT av.id AS item_id, al.group_id AS owner_id
                FROM album_versions av
                JOIN albums al ON al.id = av.album_id AND al.is_deleted = 0
                WHERE av.is_deleted = 0 AND al.group_id IS NOT NULL {region_filter}
                UNION ALL
                SELECT d.id AS item_id, al.group_id AS owner_id
                FROM digipacks d
                JOIN albums al ON al.id = d.album_id AND al.is_deleted = 0
                WHERE d.is_deleted = 0 AND al.group_id IS NOT NULL {region_filter_d}
                UNION ALL
                SELECT ls.id AS item_id, ls.group_id AS owner_id
                FROM lightsticks ls
                WHERE ls.is_deleted = 0 AND ls.group_id IS NOT NULL {region_filter_ls}
                UNION ALL
                SELECT fk.id AS item_id, fk.group_id AS owner_id
                FROM fanclub_kits fk
                WHERE fk.is_deleted = 0 AND fk.group_id IS NOT NULL {region_filter_fk}
                {photocard_union}
            ) total_items ON total_items.owner_id = g.id
            LEFT JOIN (
                SELECT uc.album_version_id AS item_id, al.group_id AS owner_id
                FROM user_collection uc
                JOIN album_versions av ON av.id = uc.album_version_id {region_filter}
                JOIN albums al ON al.id = av.album_id AND al.group_id IS NOT NULL
                WHERE uc.album_version_id IS NOT NULL
                UNION ALL
                SELECT uc.digipack_id AS item_id, al.group_id AS owner_id
                FROM user_collection uc
                JOIN digipacks d ON d.id = uc.digipack_id {region_filter_d}
                JOIN albums al ON al.id = d.album_id AND al.group_id IS NOT NULL
                WHERE uc.digipack_id IS NOT NULL
                UNION ALL
                SELECT uc.lightstick_id AS item_id, ls.group_id AS owner_id
                FROM user_collection uc
                JOIN lightsticks ls ON ls.id = uc.lightstick_id AND ls.group_id IS NOT NULL {region_filter_ls}
                WHERE uc.lightstick_id IS NOT NULL
                UNION ALL
                SELECT uc.fanclub_kit_id AS item_id, fk.group_id AS owner_id
                FROM user_collection uc
                JOIN fanclub_kits fk ON fk.id = uc.fanclub_kit_id AND fk.group_id IS NOT NULL {region_filter_fk}
                WHERE uc.fanclub_kit_id IS NOT NULL
                {owned_photocard_union}
            ) owned_items ON owned_items.owner_id = g.id
            WHERE g.is_deleted = 0
            {search_filter}
            {agency_filter}
            GROUP BY g.id
            {having}",
        );

        diesel::sql_query(sql).load::<GroupSummaryRow>(self.conn)
    }
}
