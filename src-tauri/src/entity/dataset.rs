use serde::{Deserialize, Serialize};

use crate::entity::{AlbumDto, GroupDto, LightstickDto};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DatasetDto {
    #[serde(rename = "datasetVersion")]
    pub dataset_version: String,
    #[serde(rename = "generatedAt")]
    pub generated_at: String,
    #[serde(default)]
    pub groups: Vec<GroupDto>,
    #[serde(default)]
    pub albums: Vec<AlbumDto>,
    #[serde(default)]
    pub lightsticks: Vec<LightstickDto>,
}
