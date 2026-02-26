use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// La structure de données unifiée pour un Lightstick.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LightstickDto {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "releaseYear")]
    pub release_year: i32,
    pub image: String,
    pub verified: bool,
    #[serde(rename = "groupId")]
    pub group_id: String,
}

impl LightstickDto {
    pub fn to_entity(&self) -> NewLightstick<'_> {
        NewLightstick {
            id: &self.id,
            name: &self.name,
            version: &self.version,
            release_year: self.release_year,
            image: &self.image,
            verified: self.verified,
            group_id: &self.group_id,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::lightsticks)]
pub struct NewLightstick<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub version: &'a str,
    pub release_year: i32,
    pub image: &'a str,
    pub verified: bool,
    pub group_id: &'a str,
}

#[derive(Queryable, Selectable, Insertable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::lightsticks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(base_query = lightsticks::table.order_by(lightsticks::name.asc()))]
pub struct Lightstick {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub release_year: Option<i32>,
    pub image: Option<String>,
    pub verified: bool,
    pub group_id: String,
}

impl Lightstick {
    pub fn to_dto(&self) -> LightstickDto {
        LightstickDto {
            id: self.id.clone().unwrap_or_default(),
            name: self.name.clone().unwrap_or_default(),
            version: self.version.clone().unwrap_or_default(),
            release_year: self.release_year.clone().unwrap_or_default(),
            image: self.image.clone().unwrap_or_default(),
            verified: self.verified.clone(),
            group_id: self.group_id.clone(),
        }
    }
}
