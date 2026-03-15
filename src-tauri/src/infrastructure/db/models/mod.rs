mod agency;
mod album;
mod album_version;
mod artist;
mod artist_alias;
mod digipack;
mod fanclub_kit;
mod group;
mod group_member;
mod lightstick;
mod photocard;
mod user_collection;
mod user_favorites;

pub use self::{
    agency::Agency,
    album::Album,
    album_version::{AlbumVersion, VersionFormat},
    artist::Artist,
    artist_alias::ArtistAlias,
    digipack::Digipack,
    fanclub_kit::FanclubKit,
    group::Group,
    group_member::GroupMember,
    lightstick::Lightstick,
    photocard::{Photocard, PhotocardKind},
    user_collection::{UserCollection, UserCollectionItemKind},
    user_favorites::{UserFavoriteArtist, UserFavoriteGroup},
};

// ─── Structs composites (pour les queries avec jointures) ─────────────────────

pub struct GroupWithAgency {
    pub group: Group,
    pub agency: Agency,
}

pub struct ArtistWithAliases {
    pub artist: Artist,
    pub aliases: Vec<ArtistAlias>,
}

impl ArtistWithAliases {
    /// Retourne le nom à afficher selon le contexte
    pub fn display_name(&self, kind: &str) -> Option<&str> {
        self.aliases
            .iter()
            .find(|a| a.kind == kind && a.is_primary())
            .or_else(|| self.aliases.iter().find(|a| a.kind == kind))
            .map(|a| a.name.as_str())
    }

    pub fn group_stage_name(&self) -> Option<&str> {
        self.display_name("group_stage")
    }

    pub fn solo_stage_name(&self) -> Option<&str> {
        self.display_name("solo_stage")
    }
}

pub struct AlbumFull {
    pub album: Album,
    pub versions: Vec<AlbumVersion>,
    pub digipacks: Vec<Digipack>,
}
