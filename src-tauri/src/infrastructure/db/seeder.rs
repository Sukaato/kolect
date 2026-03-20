// src-tauri/src/infrastructure/db/seed/dataset_seeder.rs

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::collections::HashSet;
use tauri_plugin_log::log;

use crate::dto::input::DatasetDto;
use crate::infrastructure::db::models::{
    Agency, Album, AlbumVersion, Artist, ArtistAlias, Digipack, FanclubKit, Group, GroupMember,
    Lightstick, Photocard,
};

// ─── Erreur ───────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum SeederError {
    #[error("Erreur Diesel: {0}")]
    Diesel(#[from] diesel::result::Error),
}

pub type SeederResult<T> = Result<T, SeederError>;

// ─── Rapport ──────────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct SeedReport {
    pub inserted: usize,
    pub updated: usize,
    pub deleted: usize,
}

impl SeedReport {
    fn merge(&mut self, other: SeedReport) {
        self.inserted += other.inserted;
        self.updated += other.updated;
        self.deleted += other.deleted;
    }
}

// ─── Seeder ───────────────────────────────────────────────────────────────────

pub struct DatasetSeeder<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> DatasetSeeder<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Point d'entrée principal — exécute le seed complet dans une transaction.
    /// Ordre FK-safe : agencies → groups → artists → aliases → members
    ///              → albums → versions → digipacks → lightsticks → kits → photocards
    pub fn run(&mut self, dto: DatasetDto) -> SeederResult<SeedReport> {
        // Conversion DTO → models avant la transaction (move partiel)
        let agencies = dto.agencies.into_iter().map(Agency::from).collect();
        let groups = dto.groups.into_iter().map(Group::from).collect();
        let artists = dto.artists.into_iter().map(Artist::from).collect();
        let artist_aliases = dto
            .artist_aliases
            .into_iter()
            .map(ArtistAlias::from)
            .collect();
        let group_members = dto
            .group_members
            .into_iter()
            .map(GroupMember::from)
            .collect();
        let albums = dto.albums.into_iter().map(Album::from).collect();
        let album_versions = dto
            .album_versions
            .into_iter()
            .map(AlbumVersion::from)
            .collect();
        let digipacks = dto.digipacks.into_iter().map(Digipack::from).collect();
        let lightsticks = dto.lightsticks.into_iter().map(Lightstick::from).collect();
        let fanclub_kits = dto.fanclub_kits.into_iter().map(FanclubKit::from).collect();
        let photocards = dto.photocards.into_iter().map(Photocard::from).collect();

        let report = self.conn.transaction::<SeedReport, SeederError, _>(|conn| {
            let mut seeder = DatasetSeeder { conn };
            let mut report = SeedReport::default();

            report.merge(seeder.seed_agencies(agencies)?);
            report.merge(seeder.seed_groups(groups)?);
            report.merge(seeder.seed_artists(artists)?);
            report.merge(seeder.seed_artist_aliases(artist_aliases)?);
            report.merge(seeder.seed_group_members(group_members)?);
            report.merge(seeder.seed_albums(albums)?);
            report.merge(seeder.seed_album_versions(album_versions)?);
            report.merge(seeder.seed_digipacks(digipacks)?);
            report.merge(seeder.seed_lightsticks(lightsticks)?);
            report.merge(seeder.seed_fanclub_kits(fanclub_kits)?);
            report.merge(seeder.seed_photocards(photocards)?);

            Ok(report)
        });

        // Rebuild des index FTS5 hors transaction (FTS5 + transactions SQLite)
        self.rebuild_fts()?;

        Ok(report.unwrap())
    }

    fn rebuild_fts(&mut self) -> SeederResult<()> {
        // Vider et repeupler groups_fts
        diesel::sql_query("DELETE FROM groups_fts").execute(self.conn)?;
        diesel::sql_query(
            "INSERT INTO groups_fts(group_id, name)
             SELECT id, name FROM groups WHERE is_deleted = 0",
        )
        .execute(self.conn)?;

        // Vider et repeupler artists_fts
        // Le champ name agrège real_name + tous les aliases séparés par un espace
        diesel::sql_query("DELETE FROM artists_fts").execute(self.conn)?;
        diesel::sql_query(
            "INSERT INTO artists_fts(artist_id, name)
             SELECT
                 ar.id,
                 ar.real_name || ' ' || COALESCE(
                     (SELECT GROUP_CONCAT(aa.name, ' ')
                      FROM artist_aliases aa
                      WHERE aa.artist_id = ar.id AND aa.is_deleted = 0),
                     ''
                 )
             FROM artists ar
             WHERE ar.is_deleted = 0 AND ar.solo_agency_id IS NOT NULL",
        )
        .execute(self.conn)?;

        log::info!("FTS indexes rebuilt (groups_fts, artists_fts)");
        Ok(())
    }

    // ─── Agencies ─────────────────────────────────────────────────────────────

    fn seed_agencies(&mut self, items: Vec<Agency>) -> SeederResult<SeedReport> {
        use crate::infrastructure::db::schema::agencies::dsl::*;

        let incoming: HashSet<String> = items.iter().map(|r| r.id.clone()).collect();
        let existing: HashSet<String> = agencies
            .select(id)
            .load::<String>(self.conn)?
            .into_iter()
            .collect();

        let mut report = SeedReport::default();

        for item in items {
            if existing.contains(&item.id) {
                diesel::update(agencies.filter(id.eq(&item.id)))
                    .set((
                        name.eq(&item.name),
                        country.eq(&item.country),
                        image_url.eq(&item.image_url),
                        is_deleted.eq(0),
                    ))
                    .execute(self.conn)?;
                report.updated += 1;
            } else {
                diesel::insert_into(agencies)
                    .values(&item)
                    .execute(self.conn)?;
                report.inserted += 1;
            }
        }

        for gone in existing.difference(&incoming) {
            diesel::update(agencies.filter(id.eq(gone)))
                .set(is_deleted.eq(1))
                .execute(self.conn)?;
            report.deleted += 1;
        }

        Ok(report)
    }

    // ─── Groups ───────────────────────────────────────────────────────────────

    fn seed_groups(&mut self, items: Vec<Group>) -> SeederResult<SeedReport> {
        use crate::infrastructure::db::schema::groups::dsl::*;

        let incoming: HashSet<String> = items.iter().map(|r| r.id.clone()).collect();
        let existing: HashSet<String> = groups
            .select(id)
            .load::<String>(self.conn)?
            .into_iter()
            .collect();

        let mut report = SeedReport::default();

        for item in items {
            if existing.contains(&item.id) {
                diesel::update(groups.filter(id.eq(&item.id)))
                    .set((
                        name.eq(&item.name),
                        debut_date.eq(&item.debut_date),
                        agency_id.eq(&item.agency_id),
                        fandom_name.eq(&item.fandom_name),
                        image_url.eq(&item.image_url),
                        is_deleted.eq(0),
                    ))
                    .execute(self.conn)?;
                report.updated += 1;
            } else {
                diesel::insert_into(groups)
                    .values(&item)
                    .execute(self.conn)?;
                report.inserted += 1;
            }
        }

        for gone in existing.difference(&incoming) {
            diesel::update(groups.filter(id.eq(gone)))
                .set(is_deleted.eq(1))
                .execute(self.conn)?;
            report.deleted += 1;
        }

        Ok(report)
    }

    // ─── Artists ──────────────────────────────────────────────────────────────

    fn seed_artists(&mut self, items: Vec<Artist>) -> SeederResult<SeedReport> {
        use crate::infrastructure::db::schema::artists::dsl::*;

        let incoming: HashSet<String> = items.iter().map(|r| r.id.clone()).collect();
        let existing: HashSet<String> = artists
            .select(id)
            .load::<String>(self.conn)?
            .into_iter()
            .collect();

        let mut report = SeedReport::default();

        for item in items {
            if existing.contains(&item.id) {
                diesel::update(artists.filter(id.eq(&item.id)))
                    .set((
                        real_name.eq(&item.real_name),
                        birth_date.eq(&item.birth_date),
                        image_url.eq(&item.image_url),
                        solo_debut_date.eq(&item.solo_debut_date),
                        solo_agency_id.eq(&item.solo_agency_id),
                        is_deleted.eq(0),
                    ))
                    .execute(self.conn)?;
                report.updated += 1;
            } else {
                diesel::insert_into(artists)
                    .values(&item)
                    .execute(self.conn)?;
                report.inserted += 1;
            }
        }

        for gone in existing.difference(&incoming) {
            diesel::update(artists.filter(id.eq(gone)))
                .set(is_deleted.eq(1))
                .execute(self.conn)?;
            report.deleted += 1;
        }

        Ok(report)
    }

    // ─── Artist Aliases ───────────────────────────────────────────────────────

    fn seed_artist_aliases(&mut self, items: Vec<ArtistAlias>) -> SeederResult<SeedReport> {
        use crate::infrastructure::db::schema::artist_aliases::dsl::*;

        let incoming: HashSet<String> = items.iter().map(|r| r.id.clone()).collect();
        let existing: HashSet<String> = artist_aliases
            .select(id)
            .load::<String>(self.conn)?
            .into_iter()
            .collect();

        let mut report = SeedReport::default();

        for item in items {
            if existing.contains(&item.id) {
                diesel::update(artist_aliases.filter(id.eq(&item.id)))
                    .set((
                        artist_id.eq(&item.artist_id),
                        name.eq(&item.name),
                        kind.eq(&item.kind),
                        is_primary.eq(item.is_primary),
                        is_deleted.eq(0),
                    ))
                    .execute(self.conn)?;
                report.updated += 1;
            } else {
                diesel::insert_into(artist_aliases)
                    .values(&item)
                    .execute(self.conn)?;
                report.inserted += 1;
            }
        }

        for gone in existing.difference(&incoming) {
            diesel::update(artist_aliases.filter(id.eq(gone)))
                .set(is_deleted.eq(1))
                .execute(self.conn)?;
            report.deleted += 1;
        }

        Ok(report)
    }

    // ─── Group Members ────────────────────────────────────────────────────────
    // PK composite (artist_id, group_id) — pas d'id propre, pas de soft delete.

    fn seed_group_members(&mut self, items: Vec<GroupMember>) -> SeederResult<SeedReport> {
        use crate::infrastructure::db::schema::group_members::dsl::*;

        let mut report = SeedReport::default();

        for item in items {
            let exists = group_members
                .filter(
                    artist_id
                        .eq(&item.artist_id)
                        .and(group_id.eq(&item.group_id)),
                )
                .count()
                .get_result::<i64>(self.conn)?
                > 0;

            if exists {
                diesel::update(
                    group_members.filter(
                        artist_id
                            .eq(&item.artist_id)
                            .and(group_id.eq(&item.group_id)),
                    ),
                )
                .set((
                    roles.eq(&item.roles),
                    join_date.eq(&item.join_date),
                    leave_date.eq(&item.leave_date),
                ))
                .execute(self.conn)?;
                report.updated += 1;
            } else {
                diesel::insert_into(group_members)
                    .values(&item)
                    .execute(self.conn)?;
                report.inserted += 1;
            }
        }

        Ok(report)
    }

    // ─── Albums ───────────────────────────────────────────────────────────────

    fn seed_albums(&mut self, items: Vec<Album>) -> SeederResult<SeedReport> {
        use crate::infrastructure::db::schema::albums::dsl::*;

        let incoming: HashSet<String> = items.iter().map(|r| r.id.clone()).collect();
        let existing: HashSet<String> = albums
            .select(id)
            .load::<String>(self.conn)?
            .into_iter()
            .collect();

        let mut report = SeedReport::default();

        for item in items {
            if existing.contains(&item.id) {
                diesel::update(albums.filter(id.eq(&item.id)))
                    .set((
                        name.eq(&item.name),
                        release_date.eq(&item.release_date),
                        region.eq(&item.region),
                        image_url.eq(&item.image_url),
                        group_id.eq(&item.group_id),
                        artist_id.eq(&item.artist_id),
                        is_deleted.eq(0),
                    ))
                    .execute(self.conn)?;
                report.updated += 1;
            } else {
                diesel::insert_into(albums)
                    .values(&item)
                    .execute(self.conn)?;
                report.inserted += 1;
            }
        }

        for gone in existing.difference(&incoming) {
            diesel::update(albums.filter(id.eq(gone)))
                .set(is_deleted.eq(1))
                .execute(self.conn)?;
            report.deleted += 1;
        }

        Ok(report)
    }

    // ─── Album Versions ───────────────────────────────────────────────────────

    fn seed_album_versions(&mut self, items: Vec<AlbumVersion>) -> SeederResult<SeedReport> {
        use crate::infrastructure::db::schema::album_versions::dsl::*;

        let incoming: HashSet<String> = items.iter().map(|r| r.id.clone()).collect();
        let existing: HashSet<String> = album_versions
            .select(id)
            .load::<String>(self.conn)?
            .into_iter()
            .collect();

        let mut report = SeedReport::default();

        for item in items {
            if existing.contains(&item.id) {
                diesel::update(album_versions.filter(id.eq(&item.id)))
                    .set((
                        album_id.eq(&item.album_id),
                        name.eq(&item.name),
                        format.eq(&item.format),
                        release_date.eq(&item.release_date),
                        region.eq(&item.region),
                        image_url.eq(&item.image_url),
                        is_deleted.eq(0),
                    ))
                    .execute(self.conn)?;
                report.updated += 1;
            } else {
                diesel::insert_into(album_versions)
                    .values(&item)
                    .execute(self.conn)?;
                report.inserted += 1;
            }
        }

        for gone in existing.difference(&incoming) {
            diesel::update(album_versions.filter(id.eq(gone)))
                .set(is_deleted.eq(1))
                .execute(self.conn)?;
            report.deleted += 1;
        }

        Ok(report)
    }

    // ─── Digipacks ────────────────────────────────────────────────────────────

    fn seed_digipacks(&mut self, items: Vec<Digipack>) -> SeederResult<SeedReport> {
        use crate::infrastructure::db::schema::digipacks::dsl::*;

        let incoming: HashSet<String> = items.iter().map(|r| r.id.clone()).collect();
        let existing: HashSet<String> = digipacks
            .select(id)
            .load::<String>(self.conn)?
            .into_iter()
            .collect();

        let mut report = SeedReport::default();

        for item in items {
            if existing.contains(&item.id) {
                diesel::update(digipacks.filter(id.eq(&item.id)))
                    .set((
                        album_id.eq(&item.album_id),
                        artist_id.eq(&item.artist_id),
                        name.eq(&item.name),
                        release_date.eq(&item.release_date),
                        region.eq(&item.region),
                        image_url.eq(&item.image_url),
                        is_deleted.eq(0),
                    ))
                    .execute(self.conn)?;
                report.updated += 1;
            } else {
                diesel::insert_into(digipacks)
                    .values(&item)
                    .execute(self.conn)?;
                report.inserted += 1;
            }
        }

        for gone in existing.difference(&incoming) {
            diesel::update(digipacks.filter(id.eq(gone)))
                .set(is_deleted.eq(1))
                .execute(self.conn)?;
            report.deleted += 1;
        }

        Ok(report)
    }

    // ─── Lightsticks ──────────────────────────────────────────────────────────

    fn seed_lightsticks(&mut self, items: Vec<Lightstick>) -> SeederResult<SeedReport> {
        use crate::infrastructure::db::schema::lightsticks::dsl::*;

        let incoming: HashSet<String> = items.iter().map(|r| r.id.clone()).collect();
        let existing: HashSet<String> = lightsticks
            .select(id)
            .load::<String>(self.conn)?
            .into_iter()
            .collect();

        let mut report = SeedReport::default();

        for item in items {
            if existing.contains(&item.id) {
                diesel::update(lightsticks.filter(id.eq(&item.id)))
                    .set((
                        group_id.eq(&item.group_id),
                        artist_id.eq(&item.artist_id),
                        name.eq(&item.name),
                        version.eq(&item.version),
                        release_date.eq(&item.release_date),
                        image_url.eq(&item.image_url),
                        is_deleted.eq(0),
                    ))
                    .execute(self.conn)?;
                report.updated += 1;
            } else {
                diesel::insert_into(lightsticks)
                    .values(&item)
                    .execute(self.conn)?;
                report.inserted += 1;
            }
        }

        for gone in existing.difference(&incoming) {
            diesel::update(lightsticks.filter(id.eq(gone)))
                .set(is_deleted.eq(1))
                .execute(self.conn)?;
            report.deleted += 1;
        }

        Ok(report)
    }

    // ─── Fanclub Kits ─────────────────────────────────────────────────────────

    fn seed_fanclub_kits(&mut self, items: Vec<FanclubKit>) -> SeederResult<SeedReport> {
        use crate::infrastructure::db::schema::fanclub_kits::dsl::*;

        let incoming: HashSet<String> = items.iter().map(|r| r.id.clone()).collect();
        let existing: HashSet<String> = fanclub_kits
            .select(id)
            .load::<String>(self.conn)?
            .into_iter()
            .collect();

        let mut report = SeedReport::default();

        for item in items {
            if existing.contains(&item.id) {
                diesel::update(fanclub_kits.filter(id.eq(&item.id)))
                    .set((
                        group_id.eq(&item.group_id),
                        artist_id.eq(&item.artist_id),
                        name.eq(&item.name),
                        release_date.eq(&item.release_date),
                        region.eq(&item.region),
                        image_url.eq(&item.image_url),
                        is_deleted.eq(0),
                    ))
                    .execute(self.conn)?;
                report.updated += 1;
            } else {
                diesel::insert_into(fanclub_kits)
                    .values(&item)
                    .execute(self.conn)?;
                report.inserted += 1;
            }
        }

        for gone in existing.difference(&incoming) {
            diesel::update(fanclub_kits.filter(id.eq(gone)))
                .set(is_deleted.eq(1))
                .execute(self.conn)?;
            report.deleted += 1;
        }

        Ok(report)
    }

    // ─── Photocards ───────────────────────────────────────────────────────────

    fn seed_photocards(&mut self, items: Vec<Photocard>) -> SeederResult<SeedReport> {
        use crate::infrastructure::db::schema::photocards::dsl::*;

        let incoming: HashSet<String> = items.iter().map(|r| r.id.clone()).collect();
        let existing: HashSet<String> = photocards
            .select(id)
            .load::<String>(self.conn)?
            .into_iter()
            .collect();

        let mut report = SeedReport::default();

        for item in items {
            if existing.contains(&item.id) {
                diesel::update(photocards.filter(id.eq(&item.id)))
                    .set((
                        artist_id.eq(&item.artist_id),
                        album_id.eq(&item.album_id),
                        album_version_id.eq(&item.album_version_id),
                        digipack_id.eq(&item.digipack_id),
                        release_date.eq(&item.release_date),
                        region.eq(&item.region),
                        image_url.eq(&item.image_url),
                        is_deleted.eq(0),
                    ))
                    .execute(self.conn)?;
                report.updated += 1;
            } else {
                diesel::insert_into(photocards)
                    .values(&item)
                    .execute(self.conn)?;
                report.inserted += 1;
            }
        }

        for gone in existing.difference(&incoming) {
            diesel::update(photocards.filter(id.eq(gone)))
                .set(is_deleted.eq(1))
                .execute(self.conn)?;
            report.deleted += 1;
        }

        Ok(report)
    }
}
