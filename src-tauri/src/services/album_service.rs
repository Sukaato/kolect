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

    /// Retourne les infos de base de l'album + progression globale.
    pub fn get_detail(&mut self, album_id: &str) -> Result<AlbumDetailDto, RepositoryError> {
        let row = AlbumRepository::new(self.conn)
            .find_detail_by_id(album_id)?
            .ok_or_else(|| RepositoryError::NotFound(album_id.to_string()))?;

        Ok(AlbumDetailDto {
            album_id: row.album_id,
            name: row.name,
            release_date: row.release_date,
            image_url: row.image_url,
            group_id: row.group_id,
            artist_id: row.artist_id,
            owned_count: row.owned_count,
            total_count: row.total_count,
        })
    }

    /// Retourne les versions de l'album avec owned_count et has_signed.
    pub fn get_versions(
        &mut self,
        album_id: &str,
    ) -> Result<Vec<AlbumVersionItemDto>, RepositoryError> {
        let rows = AlbumVersionRepository::new(self.conn).find_by_album_id_with_owned(album_id)?;

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

    /// Retourne les digipacks de l'album avec owned_count et has_signed.
    pub fn get_digipacks(
        &mut self,
        album_id: &str,
    ) -> Result<Vec<DigipackItemDto>, RepositoryError> {
        let rows = DigipackRepository::new(self.conn).find_by_album_id_with_owned(album_id)?;

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

    /// Retourne les photocards liées à l'album avec owned_count et has_signed.
    pub fn get_photocards(
        &mut self,
        album_id: &str,
    ) -> Result<Vec<PhotocardItemDto>, RepositoryError> {
        let rows = PhotocardRepository::new(self.conn).find_by_album_id_with_owned(album_id)?;

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
