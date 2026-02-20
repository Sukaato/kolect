use crate::db::{self, DbLocation};

#[tauri::command]
pub fn init_db(path: Option<String>) -> Result<String, String> {
    let loc = match path {
        Some(p) => DbLocation::Path(p),
        None => DbLocation::Default,
    };
    db::init_db(loc)
}
