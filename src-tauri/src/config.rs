use std::env::var;

#[derive(Debug)]
pub struct AppConfig {}

impl AppConfig {
    pub fn dataset_url() -> String {
        var("DATASET_URL").expect("DATASET_URL not found in env")
    }
}
