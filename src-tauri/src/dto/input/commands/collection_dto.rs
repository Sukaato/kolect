use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSummaryParams {
    pub page: u32,
    pub per_page: u32,
    pub include_photocards: bool,
    pub include_exclusive_items: bool,
    pub search: Option<String>,
    pub agency_id: Option<String>,
}
