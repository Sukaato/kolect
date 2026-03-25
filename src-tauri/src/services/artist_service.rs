use diesel::sqlite::SqliteConnection;

use crate::db::repositories::{
    AlbumRepository, ArtistAliasRepository, ArtistRepository, FanclubKitRepository,
    LightstickRepository, RepositoryError, UserFavoriteRepository,
};
use crate::dto::output::{
    AlbumSummaryDto, ArtistAliasOutputDto, ArtistDetailDto, ArtistOutputDto, FanclubKitItemDto,
    LightstickItemDto,
};

pub struct ArtistService<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> ArtistService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn get_detail(&mut self, artist_id: &str) -> Result<ArtistDetailDto, RepositoryError> {
        let artist = ArtistRepository::new(self.conn)
            .find_by_id(artist_id)?
            .ok_or_else(|| RepositoryError::NotFound(artist_id.to_string()))?;

        let is_favorite = UserFavoriteRepository::new(self.conn).is_artist_favorite(artist_id)?;

        let aliases =
            ArtistAliasRepository::new(self.conn).find_by_artist_ids(&[artist_id.to_string()])?;

        Ok(ArtistDetailDto {
            artist: ArtistOutputDto {
                id: artist.id,
                real_name: artist.real_name,
                birth_date: artist.birth_date,
                image_url: artist.image_url,
                solo_debut_date: artist.solo_debut_date,
                solo_agency_id: artist.solo_agency_id,
                is_favorite,
            },
            aliases: aliases
                .into_iter()
                .map(|a| ArtistAliasOutputDto {
                    id: a.id.clone(),
                    artist_id: a.artist_id.clone(),
                    name: a.name.clone(),
                    kind: a.kind.clone(),
                    is_primary: a.is_primary(),
                })
                .collect(),
        })
    }

    pub fn get_album_summaries(
        &mut self,
        artist_id: &str,
        include_exclusive_items: bool,
    ) -> Result<Vec<AlbumSummaryDto>, RepositoryError> {
        let rows = AlbumRepository::new(self.conn)
            .find_summaries_by_artist_id(artist_id, include_exclusive_items)?;

        Ok(rows
            .into_iter()
            .map(|r| AlbumSummaryDto {
                album_id: r.album_id,
                name: r.name,
                release_date: r.release_date,
                image_url: r.image_url,
                versions_owned_count: r.versions_owned_count,
                versions_total_count: r.versions_total_count,
                digipacks_owned_count: r.digipacks_owned_count,
                digipacks_total_count: r.digipacks_total_count,
                photocards_owned_count: r.photocards_owned_count,
            })
            .collect())
    }

    pub fn get_lightsticks(
        &mut self,
        artist_id: &str,
        include_exclusive_items: bool,
    ) -> Result<Vec<LightstickItemDto>, RepositoryError> {
        let rows = LightstickRepository::new(self.conn)
            .find_by_artist_id_with_owned(artist_id, include_exclusive_items)?;

        Ok(rows
            .into_iter()
            .map(|r| LightstickItemDto {
                id: r.id,
                group_id: r.group_id,
                artist_id: r.artist_id,
                name: r.name,
                version: r.version,
                release_date: r.release_date,
                region: r.region,
                image_url: r.image_url,
                owned_count: r.owned_count,
            })
            .collect())
    }

    pub fn get_fanclub_kits(
        &mut self,
        artist_id: &str,
    ) -> Result<Vec<FanclubKitItemDto>, RepositoryError> {
        let rows = FanclubKitRepository::new(self.conn).find_by_artist_id_with_owned(artist_id)?;

        Ok(rows
            .into_iter()
            .map(|r| FanclubKitItemDto {
                id: r.id,
                group_id: r.group_id,
                artist_id: r.artist_id,
                name: r.name,
                release_date: r.release_date,
                region: r.region,
                image_url: r.image_url,
                owned_count: r.owned_count,
            })
            .collect())
    }
}
