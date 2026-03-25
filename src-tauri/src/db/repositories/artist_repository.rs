use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bool, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use super::RepoResult;
use crate::db::models::Artist;

// ─── Row ──────────────────────────────────────────────────────────────────────

#[derive(QueryableByName, Debug)]
pub struct ArtistSummaryRow {
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

pub struct ArtistRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> ArtistRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns a single artist by its ID.
    pub fn find_by_id(&mut self, record_id: &str) -> RepoResult<Option<Artist>> {
        use crate::db::schema::artists::dsl::*;

        Ok(artists
            .filter(id.eq(record_id))
            .filter(is_deleted.eq(0))
            .first::<Artist>(self.conn)
            .optional()?)
    }

    /// Returns multiple artists by their IDs.
    pub fn find_by_ids(&mut self, ids: &[String]) -> RepoResult<Vec<Artist>> {
        use crate::db::schema::artists::dsl::*;

        Ok(artists
            .filter(id.eq_any(ids))
            .filter(is_deleted.eq(0))
            .load::<Artist>(self.conn)?)
    }

    /// Returns the solo artist summary with owned/total counts.
    /// - owned_only              : true → HAVING owned_count > 0 (collection screen)
    /// +                           false → all solo artists (home screen)
    /// - query                   : LIKE %query% search on real name and aliases
    /// - agency_ids              : filter by solo_agency_id
    /// - include_photocards      : count photocards in total/owned
    /// - include_exclusive_items : if false, only GLOBAL region items are counted
    pub fn get_summary(
        &mut self,
        owned_only: bool,
        query: Option<&str>,
        agency_ids: Option<&[String]>,
        include_photocards: bool,
        include_exclusive_items: bool,
    ) -> Result<Vec<ArtistSummaryRow>, diesel::result::Error> {
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
                 SELECT p.id AS item_id, al.artist_id AS owner_id
                 FROM photocards p
                 JOIN album_versions av ON av.id = p.album_version_id AND av.is_deleted = 0
                 JOIN albums al ON al.id = av.album_id AND al.artist_id IS NOT NULL AND al.is_deleted = 0
                 WHERE p.is_deleted = 0 {region_filter_p}
                 UNION ALL
                 SELECT p.id AS item_id, d.artist_id AS owner_id
                 FROM photocards p
                 JOIN digipacks d ON d.id = p.digipack_id AND d.is_deleted = 0
                 WHERE p.is_deleted = 0 AND d.artist_id IS NOT NULL {region_filter_p}"
            )
        } else {
            String::new()
        };

        let owned_photocard_union = if include_photocards {
            format!(
                "UNION ALL
                 SELECT uc.photocard_id AS item_id, al.artist_id AS owner_id
                 FROM user_collection uc
                 JOIN photocards p ON p.id = uc.photocard_id {region_filter_p}
                 JOIN album_versions av ON av.id = p.album_version_id
                 JOIN albums al ON al.id = av.album_id AND al.artist_id IS NOT NULL
                 WHERE uc.photocard_id IS NOT NULL
                 UNION ALL
                 SELECT uc.photocard_id AS item_id, d.artist_id AS owner_id
                 FROM user_collection uc
                 JOIN photocards p ON p.id = uc.photocard_id {region_filter_p}
                 JOIN digipacks d ON d.id = p.digipack_id AND d.artist_id IS NOT NULL
                 WHERE uc.photocard_id IS NOT NULL"
            )
        } else {
            String::new()
        };

        let search_filter = match query {
            Some(q) => {
                let q = q.replace('\'', "''");
                format!(
                    "AND (
                        ar.real_name LIKE '%{q}%'
                        OR EXISTS (
                            SELECT 1 FROM artist_aliases aa
                            WHERE aa.artist_id = ar.id
                              AND aa.is_deleted = 0
                              AND aa.name LIKE '%{q}%'
                        )
                    )"
                )
            }
            None => String::new(),
        };

        let agency_filter = match agency_ids.filter(|ids| !ids.is_empty()) {
            Some(ids) => {
                let placeholders = ids
                    .iter()
                    .map(|id| format!("'{}'", id.replace('\'', "''")))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("AND ar.solo_agency_id IN ({placeholders})")
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
                ar.id,
                COALESCE(
                    (SELECT aa.name FROM artist_aliases aa
                     WHERE aa.artist_id = ar.id AND aa.kind = 'solo_stage' AND aa.is_primary = 1
                     AND aa.is_deleted = 0 LIMIT 1),
                    (SELECT aa.name FROM artist_aliases aa
                     WHERE aa.artist_id = ar.id AND aa.kind = 'group_stage' AND aa.is_primary = 1
                     AND aa.is_deleted = 0 LIMIT 1),
                    ar.real_name
                ) AS name,
                a.name AS agency_name,
                ar.image_url,
                CASE WHEN ufa.artist_id IS NOT NULL THEN 1 ELSE 0 END AS is_favorite,
                COUNT(DISTINCT total_items.item_id) AS total_count,
                COUNT(DISTINCT owned_items.item_id) AS owned_count
            FROM artists ar
            JOIN agencies a ON a.id = ar.solo_agency_id
            LEFT JOIN user_favorites_artists ufa ON ufa.artist_id = ar.id
            LEFT JOIN (
                SELECT av.id AS item_id, al.artist_id AS owner_id
                FROM album_versions av
                JOIN albums al ON al.id = av.album_id AND al.is_deleted = 0
                WHERE av.is_deleted = 0 AND al.artist_id IS NOT NULL {region_filter}
                UNION ALL
                SELECT d.id AS item_id, d.artist_id AS owner_id
                FROM digipacks d
                WHERE d.is_deleted = 0 AND d.artist_id IS NOT NULL {region_filter_d}
                UNION ALL
                SELECT ls.id AS item_id, ls.artist_id AS owner_id
                FROM lightsticks ls
                WHERE ls.is_deleted = 0 AND ls.artist_id IS NOT NULL {region_filter_ls}
                UNION ALL
                SELECT fk.id AS item_id, fk.artist_id AS owner_id
                FROM fanclub_kits fk
                WHERE fk.is_deleted = 0 AND fk.artist_id IS NOT NULL {region_filter_fk}
                {photocard_union}
            ) total_items ON total_items.owner_id = ar.id
            LEFT JOIN (
                SELECT uc.album_version_id AS item_id, al.artist_id AS owner_id
                FROM user_collection uc
                JOIN album_versions av ON av.id = uc.album_version_id {region_filter}
                JOIN albums al ON al.id = av.album_id AND al.artist_id IS NOT NULL
                WHERE uc.album_version_id IS NOT NULL
                UNION ALL
                SELECT uc.digipack_id AS item_id, d.artist_id AS owner_id
                FROM user_collection uc
                JOIN digipacks d ON d.id = uc.digipack_id AND d.artist_id IS NOT NULL {region_filter_d}
                WHERE uc.digipack_id IS NOT NULL
                UNION ALL
                SELECT uc.lightstick_id AS item_id, ls.artist_id AS owner_id
                FROM user_collection uc
                JOIN lightsticks ls ON ls.id = uc.lightstick_id AND ls.artist_id IS NOT NULL {region_filter_ls}
                WHERE uc.lightstick_id IS NOT NULL
                UNION ALL
                SELECT uc.fanclub_kit_id AS item_id, fk.artist_id AS owner_id
                FROM user_collection uc
                JOIN fanclub_kits fk ON fk.id = uc.fanclub_kit_id AND fk.artist_id IS NOT NULL {region_filter_fk}
                WHERE uc.fanclub_kit_id IS NOT NULL
                {owned_photocard_union}
            ) owned_items ON owned_items.owner_id = ar.id
            WHERE ar.is_deleted = 0
              AND ar.solo_agency_id IS NOT NULL
            {search_filter}
            {agency_filter}
            GROUP BY ar.id
            {having}",
        );

        diesel::sql_query(sql).load::<ArtistSummaryRow>(self.conn)
    }
}
