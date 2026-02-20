use crate::db::{self, CollectionItem, DbLocation, ProductType};
use std::str::FromStr;

#[tauri::command]
pub fn add_to_collection(
    path: Option<String>,
    product_id: String,
    product_type: String,
) -> Result<String, String> {
    let loc = match path {
        Some(p) => DbLocation::Path(p),
        None => DbLocation::Default,
    };
    let pt = ProductType::from_str(&product_type)?;
    db::add_to_collection(loc, &product_id, pt)
}

#[tauri::command]
pub fn get_collection(path: Option<String>) -> Result<Vec<CollectionItem>, String> {
    let loc = match path {
        Some(p) => DbLocation::Path(p),
        None => DbLocation::Default,
    };
    db::get_collection(loc)
}

#[tauri::command]
pub fn remove_from_collection(path: Option<String>, id: String) -> Result<bool, String> {
    let loc = match path {
        Some(p) => DbLocation::Path(p),
        None => DbLocation::Default,
    };
    db::remove_from_collection(loc, &id)
}
