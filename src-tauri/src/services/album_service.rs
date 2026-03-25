use diesel::sqlite::SqliteConnection;

use crate::db::repositories::{
    AlbumRepository, AlbumVersionRepository, DigipackRepository, PhotocardRepository,
    RepositoryError,
};
use crate::dto::output::{AlbumDetailDto, AlbumVersionItemDto, DigipackItemDto, PhotocardItemDto};

pub struct AlbumService<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> AlbumService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Returns album info with separate owned/total counts for versions, digipacks and photocards.
    pub fn get_detail(
        &mut self,
        album_id: &str,
        include_exclusive_items: bool,
    ) -> Result<AlbumDetailDto, RepositoryError> {
        let row = AlbumRepository::new(self.conn)
            .find_detail_by_id(album_id, include_exclusive_items)?
            .ok_or_else(|| RepositoryError::NotFound(album_id.to_string()))?;

        Ok(AlbumDetailDto {
            album_id: row.album_id,
            name: row.name,
            release_date: row.release_date,
            image_url: row.image_url,
            group_id: row.group_id,
            artist_id: row.artist_id,
            versions_owned_count: row.versions_owned_count,
            versions_total_count: row.versions_total_count,
            digipacks_owned_count: row.digipacks_owned_count,
            digipacks_total_count: row.digipacks_total_count,
            photocards_owned_count: row.photocards_owned_count,
            photocards_total_count: row.photocards_total_count,
        })
    }

    /// Returns album versions with owned_count and has_signed.
    pub fn get_versions(
        &mut self,
        album_id: &str,
        include_exclusive_items: bool,
    ) -> Result<Vec<AlbumVersionItemDto>, RepositoryError> {
        let rows = AlbumVersionRepository::new(self.conn)
            .find_by_album_id_with_owned(album_id, include_exclusive_items)?;

        Ok(rows
            .into_iter()
            .map(|r| AlbumVersionItemDto {
                id: r.id,
                name: r.name,
                format: r.format,
                release_date: r.release_date,
                region: r.region,
                image_url: r.image_url,
                owned_count: r.owned_count,
                has_signed: r.has_signed,
            })
            .collect())
    }

    /// Returns album digipacks with owned_count and has_signed.
    pub fn get_digipacks(
        &mut self,
        album_id: &str,
        include_exclusive_items: bool,
    ) -> Result<Vec<DigipackItemDto>, RepositoryError> {
        let rows = DigipackRepository::new(self.conn)
            .find_by_album_id_with_owned(album_id, include_exclusive_items)?;

        Ok(rows
            .into_iter()
            .map(|r| DigipackItemDto {
                id: r.id,
                name: r.name,
                artist_id: r.artist_id,
                release_date: r.release_date,
                region: r.region,
                image_url: r.image_url,
                owned_count: r.owned_count,
                has_signed: r.has_signed,
            })
            .collect())
    }

    /// Returns photocards linked to the album with owned_count and has_signed.
    pub fn get_photocards(
        &mut self,
        album_id: &str,
        include_exclusive_items: bool,
    ) -> Result<Vec<PhotocardItemDto>, RepositoryError> {
        let rows = PhotocardRepository::new(self.conn)
            .find_by_album_id_with_owned(album_id, include_exclusive_items)?;

        Ok(rows
            .into_iter()
            .map(|r| PhotocardItemDto {
                id: r.id,
                artist_id: r.artist_id,
                album_version_id: r.album_version_id,
                digipack_id: r.digipack_id,
                region: r.region,
                image_url: r.image_url,
                owned_count: r.owned_count,
                has_signed: r.has_signed,
            })
            .collect())
    }
}
