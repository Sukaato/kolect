use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSummaryItem {
    pub id: String,
    /// "group" | "solo"
    pub kind: String,
    pub name: String,
    pub agency_name: String,
    pub image_url: Option<String>,
    pub is_favorite: bool,
    pub owned_count: i64,
    pub total_count: i64,
}
