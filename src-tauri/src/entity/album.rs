use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// JSon struct for album data returned dataset
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlbumDto {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    #[serde(rename = "coverImage")]
    pub cover_image: String,
    pub barcode: String,
    pub verified: bool,
    #[serde(rename = "groupId")]
    pub group_id: String,
}

impl AlbumDto {
    pub fn to_entity(&self) -> NewAlbum<'_> {
        NewAlbum {
            id: &self.id,
            title: &self.title,
            kind: &self.kind,
            release_date: &self.release_date,
            cover_image: &self.cover_image,
            barcode: &self.barcode,
            verified: self.verified,
            group_id: &self.group_id,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::albums)]
pub struct NewAlbum<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub kind: &'a str,
    pub release_date: &'a str,
    pub cover_image: &'a str,
    pub barcode: &'a str,
    pub verified: bool,
    pub group_id: &'a str,
}

#[derive(Queryable, Selectable, Insertable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(base_query = albums::table.order_by(albums::title.asc()))]
pub struct Album {
    pub id: Option<String>,
    pub title: String,
    pub kind: Option<String>,
    pub release_date: Option<String>,
    pub cover_image: Option<String>,
    pub barcode: Option<String>,
    pub verified: bool,
    pub group_id: String,
}

impl Album {
    pub fn from_dto(dto: &AlbumDto) -> Self {
        Album {
            id: Some(dto.id.clone()),
            title: dto.title.clone(),
            kind: Some(dto.kind.clone()),
            release_date: Some(dto.release_date.clone()),
            cover_image: Some(dto.cover_image.clone()),
            barcode: Some(dto.barcode.clone()),
            verified: dto.verified.clone(),
            group_id: dto.group_id.clone(),
        }
    }

    pub fn to_dto(&self) -> AlbumDto {
        AlbumDto {
            id: self.id.clone().unwrap_or_default(),
            title: self.title.clone(),
            kind: self.kind.clone().unwrap_or_default(),
            release_date: self.release_date.clone().unwrap_or_default(),
            cover_image: self.cover_image.clone().unwrap_or_default(),
            barcode: self.barcode.clone().unwrap_or_default(),
            verified: self.verified.clone(),
            group_id: self.group_id.clone(),
        }
    }
}
