use crate::db::DbLocation;
use crate::services::database::{self, ProductType};
use crate::services::logger;
use std::str::FromStr;

/// Add item to collection
#[tauri::command]
pub fn add_to_collection(
    path: Option<String>,
    product_id: String,
    product_type: String,
) -> Result<String, String> {
    logger::info(
        "[add_to_collection] Adding to collection",
        Some(&format!("product_id={}, type={}", product_id, product_type)),
    );

    let loc = match path {
        Some(p) => DbLocation::Path(p),
        None => DbLocation::Default,
    };
    let pt = ProductType::from_str(&product_type)?;
    database::add_to_collection(&loc, &product_id, pt)
}

/// Get collection items
#[tauri::command]
pub fn get_collection(path: Option<String>) -> Result<Vec<database::CollectionItem>, String> {
    logger::info("[get_collection] Fetching collection items", None);

    let loc = match path {
        Some(p) => DbLocation::Path(p),
        None => DbLocation::Default,
    };
    database::get_collection(&loc)
}

/// Remove item from collection
#[tauri::command]
pub fn remove_from_collection(path: Option<String>, id: String) -> Result<bool, String> {
    logger::info("[remove_from_collection] Removing item", Some(&id));

    let loc = match path {
        Some(p) => DbLocation::Path(p),
        None => DbLocation::Default,
    };
    database::remove_from_collection(&loc, &id)
}
