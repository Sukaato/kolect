use crate::infrastructure::db::models::Agency;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgencyDto {
    pub id: String,
    pub name: String,
}

impl From<Agency> for AgencyDto {
    fn from(agency: Agency) -> Self {
        Self {
            id: agency.id,
            name: agency.name,
        }
    }
}
