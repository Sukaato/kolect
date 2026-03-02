use crate::entity::{CollectibleDto, GroupDto};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasetDto {
    #[serde(rename = "datasetVersion")]
    pub dataset_version: String,

    #[serde(rename = "generatedAt")]
    pub generated_at: String,

    #[serde(default)]
    pub groups: Vec<GroupDto>,

    #[serde(default)]
    pub collectibles: Vec<CollectibleDto>,
}
