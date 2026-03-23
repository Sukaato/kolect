use diesel::sqlite::SqliteConnection;

use crate::db::repositories::{
    ArtistRepository, ArtistSummaryRow, GroupRepository, GroupSummaryRow, Page, PaginatedResult,
};
use crate::dto::output::CollectionSummaryItem;

// ─── Service ──────────────────────────────────────────────────────────────────

pub struct CollectionService<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> CollectionService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns the paginated collection summary — groups and solo artists unified.
    /// Sorted: favorites first, then by name ASC.
    /// include_photocards: if true, photocards are counted in total/owned.
    pub fn get_summary(
        &mut self,
        page: Page,
        search: Option<&str>,
        agency_id: Option<&str>,
        include_photocards: bool,
    ) -> Result<PaginatedResult<CollectionSummaryItem>, diesel::result::Error> {
        let agency_ids: Option<Vec<String>> = agency_id.map(|id| vec![id.to_string()]);

        let mut groups =
            self.get_groups_summary(search, agency_ids.as_deref(), include_photocards)?;
        let mut artists =
            self.get_artists_summary(search, agency_ids.as_deref(), include_photocards)?;

        // Merge and convert
        let mut items: Vec<CollectionSummaryItem> = groups
            .drain(..)
            .map(|r| CollectionSummaryItem {
                id: r.id,
                kind: "group".to_string(),
                name: r.name,
                agency_name: r.agency_name,
                image_url: r.image_url,
                is_favorite: r.is_favorite,
                owned_count: r.owned_count,
                total_count: r.total_count,
            })
            .chain(artists.drain(..).map(|r| CollectionSummaryItem {
                id: r.id,
                kind: "solo".to_string(),
                name: r.name,
                agency_name: r.agency_name,
                image_url: r.image_url,
                is_favorite: r.is_favorite,
                owned_count: r.owned_count,
                total_count: r.total_count,
            }))
            .collect();

        // Sort: favorites first, then by name ASC
        items.sort_by(|a, b| {
            b.is_favorite
                .cmp(&a.is_favorite)
                .then_with(|| a.name.cmp(&b.name))
        });

        let total = items.len() as i64;

        // In-memory pagination (data already loaded for cross-table sorting)
        let offset = page.offset() as usize;
        let limit = page.limit() as usize;
        let data: Vec<CollectionSummaryItem> = items.into_iter().skip(offset).take(limit).collect();

        Ok(PaginatedResult::new(data, page, total))
    }

    // ─── Private helpers ──────────────────────────────────────────────────────

    fn get_groups_summary(
        &mut self,
        query: Option<&str>,
        agency_ids: Option<&[String]>,
        include_photocards: bool,
    ) -> Result<Vec<GroupSummaryRow>, diesel::result::Error> {
        GroupRepository::new(self.conn).get_summary(true, query, agency_ids, include_photocards)
    }

    fn get_artists_summary(
        &mut self,
        query: Option<&str>,
        agency_ids: Option<&[String]>,
        include_photocards: bool,
    ) -> Result<Vec<ArtistSummaryRow>, diesel::result::Error> {
        ArtistRepository::new(self.conn).get_summary(true, query, agency_ids, include_photocards)
    }
}
