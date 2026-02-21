pub mod schema;

use rusqlite::Connection;
use std::path::PathBuf;

/// Database location - either default (in app data) or custom path
pub enum DbLocation {
    Default,
    Path(String),
}

impl DbLocation {
    fn get_path(&self) -> PathBuf {
        match self {
            DbLocation::Default => {
                if let Some(data_dir) = dirs::config_dir() {
                    data_dir.join("kolect").join("collection.db")
                } else {
                    PathBuf::from("collection.db")
                }
            }
            DbLocation::Path(p) => PathBuf::from(p),
        }
    }

    pub fn connect(&self) -> Result<Connection, String> {
        let path = self.get_path();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        Connection::open(&path).map_err(|e| e.to_string())
    }
}

/// Initialize database with schema
pub fn init(location: &DbLocation) -> Result<(), String> {
    let conn = location.connect()?;
    conn.execute_batch(schema::SCHEMA)
        .map_err(|e| e.to_string())
}
