// src-tauri/src/services/collection_service.rs

use diesel::sqlite::SqliteConnection;
use serde::Serialize;

use crate::infrastructure::db::repositories::{
    artist_repository::ArtistSummaryRow, group_repository::GroupSummaryRow, ArtistRepository,
    GroupRepository, Page, PaginatedResult,
};

// ─── Paramètres de tri ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum CollectionSortBy {
    Name,
    Agency,
}

// ─── DTO de sortie ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSummaryItem {
    pub id: String,
    /// "group" | "solo"
    pub kind: String,
    pub name: String,
    pub agency_name: String,
    pub image_url: Option<String>,
    pub is_favorite: bool,
    pub owned_count: i64,
    pub total_count: i64,
}

// ─── Résultats des raw queries ────────────────────────────────────────────────

// ─── Service ──────────────────────────────────────────────────────────────────

pub struct CollectionService<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> CollectionService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Retourne le sommaire paginé de la collection — groupes + solos unifiés.
    /// Triés : favoris d'abord, puis par nom ASC ou agence ASC.
    /// include_photocards : si true, les photocards sont comptées dans total/owned.
    pub fn get_summary(
        &mut self,
        page: Page,
        sort_by: CollectionSortBy,
        include_photocards: bool,
    ) -> Result<PaginatedResult<CollectionSummaryItem>, diesel::result::Error> {
        let mut groups = self.get_groups_summary(None, None, include_photocards)?;
        let mut artists = self.get_artists_summary(None, None, include_photocards)?;

        // Fusionner et convertir
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

        // Tri : favoris d'abord, puis critère secondaire ASC
        items.sort_by(|a, b| {
            b.is_favorite
                .cmp(&a.is_favorite)
                .then_with(|| match sort_by {
                    CollectionSortBy::Name => a.name.cmp(&b.name),
                    CollectionSortBy::Agency => a
                        .agency_name
                        .cmp(&b.agency_name)
                        .then_with(|| a.name.cmp(&b.name)),
                })
        });

        let total = items.len() as i64;

        // Pagination en mémoire (données déjà chargées pour le tri cross-table)
        let offset = page.offset() as usize;
        let limit = page.limit() as usize;
        let data: Vec<CollectionSummaryItem> = items.into_iter().skip(offset).take(limit).collect();

        Ok(PaginatedResult::new(data, page, total))
    }

    // ─── Sous-méthodes privées ────────────────────────────────────────────────

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
