use crate::db::{self, DbLocation};
use crate::services::logger;

/// Initialize database
#[tauri::command]
pub fn init_db(path: Option<String>) -> Result<String, String> {
    logger::info("[init_db] Initializing database...", path.as_deref());

    let loc = match path {
        Some(p) => DbLocation::Path(p),
        None => DbLocation::Default,
    };

    db::init(&loc)?;
    Ok("Database initialized successfully".to_string())
}
