use chrono::NaiveDate;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::{Sqlite, SqliteValue};
use diesel::{deserialize, serialize};
use serde::{Deserialize, Serialize};

/// ===========================================
/// ENUM TYPE
/// ===========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
#[serde(rename_all = "snake_case")]
pub enum CollectibleType {
    Album,
    Lightstick,
    Merch,
    Photocard,
}

/// ===========================================
/// Diesel TEXT <-> Enum
/// ===========================================
impl CollectibleType {
    fn as_str(&self) -> &'static str {
        match self {
            CollectibleType::Album => "album",
            CollectibleType::Lightstick => "lightstick",
            CollectibleType::Merch => "merch",
            CollectibleType::Photocard => "photocard",
        }
    }
}

impl ToSql<Text, Sqlite> for CollectibleType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.as_str());
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for CollectibleType {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;

        match value.as_str() {
            "album" => Ok(Self::Album),
            "lightstick" => Ok(Self::Lightstick),
            "merch" => Ok(Self::Merch),
            "photocard" => Ok(Self::Photocard),
            _ => Err(format!("Unknown collectible type: {}", value).into()),
        }
    }
}

/// ===========================================
/// DTO (MATCH EXACT JSON)
/// ===========================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectibleDto {
    pub id: String,
    pub name: String,
    pub kind: CollectibleType,

    #[serde(rename = "releaseDate")]
    pub release_date: Option<NaiveDate>,

    pub version: Option<String>,
    pub barcode: Option<String>,
    pub image: Option<String>,
    pub verified: bool,

    #[serde(rename = "groupId")]
    pub group_id: String,
}

/// ===========================================
/// ENTITY (DB)
/// ===========================================
#[derive(Queryable, Insertable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::collectibles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Collectible {
    pub id: String,
    pub name: String,
    pub kind: CollectibleType,
    pub release_date: Option<NaiveDate>,
    pub version: Option<String>,
    pub barcode: Option<String>,
    pub image: Option<String>,
    pub verified: bool,
    pub group_id: String,
}

/// ===========================================
/// MAPPING
/// ===========================================
impl From<CollectibleDto> for Collectible {
    fn from(dto: CollectibleDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            kind: dto.kind,
            release_date: dto.release_date,
            version: dto.version,
            barcode: dto.barcode,
            image: dto.image,
            verified: dto.verified,
            group_id: dto.group_id,
        }
    }
}

impl From<Collectible> for CollectibleDto {
    fn from(entity: Collectible) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            kind: entity.kind,
            release_date: entity.release_date,
            version: entity.version,
            barcode: entity.barcode,
            image: entity.image,
            verified: entity.verified,
            group_id: entity.group_id,
        }
    }
}
