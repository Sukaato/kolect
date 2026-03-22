use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use uuid::Uuid;

use super::RepoResult;
use crate::dto::input::commands::CollectibleType;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::infrastructure::db::schema::user_collection)]
pub struct CollectionRow {
    pub id: String,
    pub album_id: Option<String>,
    pub album_version_id: Option<String>,
    pub digipack_id: Option<String>,
    pub lightstick_id: Option<String>,
    pub fanclub_kit_id: Option<String>,
    pub photocard_id: Option<String>,
    pub acquired_at: String,
    pub notes: Option<String>,
    pub is_signed: i32,
}

pub struct CollectionRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> CollectionRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Retourne toutes les lignes pour un item donné, triées par acquired_at ASC.
    pub fn find_by_item(
        &mut self,
        item_type: &CollectibleType,
        item_id: &str,
    ) -> RepoResult<Vec<CollectionRow>> {
        use crate::infrastructure::db::schema::user_collection::dsl::*;

        let rows = match item_type {
            CollectibleType::Album => user_collection
                .filter(album_id.eq(item_id))
                .order(acquired_at.asc())
                .select(CollectionRow::as_select())
                .load(self.conn)?,
            CollectibleType::AlbumVersion => user_collection
                .filter(album_version_id.eq(item_id))
                .order(acquired_at.asc())
                .select(CollectionRow::as_select())
                .load(self.conn)?,
            CollectibleType::Digipack => user_collection
                .filter(digipack_id.eq(item_id))
                .order(acquired_at.asc())
                .select(CollectionRow::as_select())
                .load(self.conn)?,
            CollectibleType::Lightstick => user_collection
                .filter(lightstick_id.eq(item_id))
                .order(acquired_at.asc())
                .select(CollectionRow::as_select())
                .load(self.conn)?,
            CollectibleType::FanclubKit => user_collection
                .filter(fanclub_kit_id.eq(item_id))
                .order(acquired_at.asc())
                .select(CollectionRow::as_select())
                .load(self.conn)?,
            CollectibleType::Photocard => user_collection
                .filter(photocard_id.eq(item_id))
                .order(acquired_at.asc())
                .select(CollectionRow::as_select())
                .load(self.conn)?,
        };

        Ok(rows)
    }

    /// Insère une nouvelle ligne pour un item.
    pub fn add(&mut self, item_type: &CollectibleType, item_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::user_collection::dsl::*;

        let new_id = Uuid::new_v4().to_string();

        match item_type {
            CollectibleType::Album => diesel::insert_into(user_collection)
                .values((id.eq(&new_id), album_id.eq(item_id)))
                .execute(self.conn)?,
            CollectibleType::AlbumVersion => diesel::insert_into(user_collection)
                .values((id.eq(&new_id), album_version_id.eq(item_id)))
                .execute(self.conn)?,
            CollectibleType::Digipack => diesel::insert_into(user_collection)
                .values((id.eq(&new_id), digipack_id.eq(item_id)))
                .execute(self.conn)?,
            CollectibleType::Lightstick => diesel::insert_into(user_collection)
                .values((id.eq(&new_id), lightstick_id.eq(item_id)))
                .execute(self.conn)?,
            CollectibleType::FanclubKit => diesel::insert_into(user_collection)
                .values((id.eq(&new_id), fanclub_kit_id.eq(item_id)))
                .execute(self.conn)?,
            CollectibleType::Photocard => diesel::insert_into(user_collection)
                .values((id.eq(&new_id), photocard_id.eq(item_id)))
                .execute(self.conn)?,
        };

        Ok(())
    }

    /// Supprime une ligne par son id.
    pub fn remove(&mut self, row_id: &str) -> RepoResult<()> {
        use crate::infrastructure::db::schema::user_collection::dsl::*;

        diesel::delete(user_collection.filter(id.eq(row_id))).execute(self.conn)?;

        Ok(())
    }

    /// Met à jour is_signed sur une ligne par son id.
    pub fn set_signed(&mut self, row_id: &str, signed: bool) -> RepoResult<()> {
        use crate::infrastructure::db::schema::user_collection::dsl::*;

        diesel::update(user_collection.filter(id.eq(row_id)))
            .set(is_signed.eq(signed as i32))
            .execute(self.conn)?;

        Ok(())
    }
}
