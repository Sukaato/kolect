use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// La structure de données unifiée pour un Album.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupDto {
    pub id: String,
    pub name: String,
    pub agency: String,
    #[serde(rename = "debutYear")]
    pub debut_year: i32,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}

impl GroupDto {
    pub fn to_entity(&self) -> NewGroup<'_> {
        NewGroup {
            id: &self.id,
            name: &self.name,
            agency: &self.agency,
            debut_year: self.debut_year,
            is_active: self.is_active,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::groups)]
pub struct NewGroup<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub agency: &'a str,
    pub debut_year: i32,
    pub is_active: bool,
}

#[derive(Queryable, Selectable, Insertable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(base_query = groups::table.order_by(groups::name.asc()))]
pub struct Group {
    pub id: Option<String>,
    pub name: String,
    pub agency: Option<String>,
    pub debut_year: Option<i32>,
    pub is_active: bool,
}

impl Group {
    pub fn to_dto(&self) -> GroupDto {
        GroupDto {
            id: self.id.clone().unwrap_or_default(),
            name: self.name.clone(),
            agency: self.agency.clone().unwrap_or_default(),
            debut_year: self.debut_year.unwrap_or_default(),
            is_active: self.is_active,
        }
    }
}
