use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;

/// Database location - either default (in app data) or custom path
pub enum DbLocation {
    Default,
    Path(String),
}

impl DbLocation {
    fn get_path(&self) -> PathBuf {
        match self {
            DbLocation::Default => {
                // Get the default app data directory
                if let Some(data_dir) = dirs::config_dir() {
                    data_dir.join("kolect").join("collection.db")
                } else {
                    PathBuf::from("collection.db")
                }
            }
            DbLocation::Path(p) => PathBuf::from(p),
        }
    }

    fn connect(&self) -> Result<Connection, String> {
        let path = self.get_path();

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        Connection::open(&path).map_err(|e| e.to_string())
    }
}

/// Product type enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductType {
    Album,
    Lightstick,
}

impl ProductType {
    pub fn as_str(&self) -> &str {
        match self {
            ProductType::Album => "ALBUM",
            ProductType::Lightstick => "LIGHTSTICK",
        }
    }
}

impl FromStr for ProductType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ALBUM" => Ok(ProductType::Album),
            "LIGHTSTICK" => Ok(ProductType::Lightstick),
            _ => Err(format!("Invalid product type: {}", s)),
        }
    }
}

/// Collection item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionItem {
    pub id: String,
    pub product_id: String,
    pub product_type: String,
    pub added_at: i64,
}

/// Initialize database with schema
pub fn init_db(location: DbLocation) -> Result<String, String> {
    let conn = location.connect()?;

    let schema = r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS groups (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            agency TEXT,
            debut_year INTEGER,
            is_active BOOLEAN
        );

        CREATE TABLE IF NOT EXISTS albums (
            id TEXT PRIMARY KEY,
            group_id TEXT NOT NULL,
            title TEXT NOT NULL,
            type TEXT CHECK (type IN ('EP', 'Album', 'Single')),
            release_date TEXT,
            cover_image TEXT,
            barcode TEXT,
            verified BOOLEAN,
            FOREIGN KEY (group_id) REFERENCES groups (id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS lightsticks (
            id TEXT PRIMARY KEY,
            group_id TEXT NOT NULL,
            name TEXT,
            version TEXT,
            release_year INTEGER,
            image TEXT,
            verified BOOLEAN,
            FOREIGN KEY (group_id) REFERENCES groups (id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS user_collection (
            id TEXT PRIMARY KEY,
            product_id TEXT NOT NULL,
            product_type TEXT CHECK (product_type IN ('ALBUM', 'LIGHTSTICK')),
            added_at INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_albums_group ON albums (group_id);
        CREATE INDEX IF NOT EXISTS idx_lightsticks_group ON lightsticks (group_id);
        CREATE INDEX IF NOT EXISTS idx_collection_product ON user_collection (product_id);
    "#;

    conn.execute_batch(schema).map_err(|e| e.to_string())?;

    Ok("Database initialized successfully".to_string())
}

/// Add item to collection
pub fn add_to_collection(
    location: DbLocation,
    product_id: &str,
    product_type: ProductType,
) -> Result<String, String> {
    let conn = location.connect()?;
    let id = Uuid::new_v4().to_string();
    let added_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() as i64;

    conn.execute(
        "INSERT INTO user_collection (id, product_id, product_type, added_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![&id, product_id, product_type.as_str(), added_at],
    )
    .map_err(|e| e.to_string())?;

    Ok(id)
}

/// Get collection items
pub fn get_collection(location: DbLocation) -> Result<Vec<CollectionItem>, String> {
    let conn = location.connect()?;

    let mut stmt = conn
        .prepare("SELECT id, product_id, product_type, added_at FROM user_collection ORDER BY added_at DESC")
        .map_err(|e| e.to_string())?;

    let items = stmt
        .query_map([], |row| {
            Ok(CollectionItem {
                id: row.get(0)?,
                product_id: row.get(1)?,
                product_type: row.get(2)?,
                added_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<SqlResult<Vec<_>>>()
        .map_err(|e| e.to_string())?;

    Ok(items)
}

/// Remove item from collection
pub fn remove_from_collection(location: DbLocation, id: &str) -> Result<bool, String> {
    let conn = location.connect()?;

    let affected = conn
        .execute(
            "DELETE FROM user_collection WHERE id = ?1",
            rusqlite::params![id],
        )
        .map_err(|e| e.to_string())?;

    Ok(affected > 0)
}
