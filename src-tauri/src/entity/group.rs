use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// ===============================
/// DTO (JSON)
/// ===============================
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

/// ===============================
/// Entity (DB)
/// ===============================
#[derive(Queryable, Insertable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Group {
    pub id: String,
    pub name: String,
    pub agency: String,
    pub debut_year: i32,
    pub is_active: bool,
}

/// ===============================
/// Mapping
/// ===============================
impl From<GroupDto> for Group {
    fn from(dto: GroupDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            agency: dto.agency,
            debut_year: dto.debut_year,
            is_active: dto.is_active,
        }
    }
}

impl From<Group> for GroupDto {
    fn from(entity: Group) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            agency: entity.agency,
            debut_year: entity.debut_year,
            is_active: entity.is_active,
        }
    }
}
