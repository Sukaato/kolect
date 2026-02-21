/// Database schema and entity types

#[derive(Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub agency: Option<String>,
    pub debut_year: Option<i64>,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct Album {
    pub id: String,
    pub group_id: String,
    pub title: String,
    pub album_type: String,
    pub release_date: Option<String>,
    pub cover_image: Option<String>,
    pub barcode: Option<String>,
    pub verified: bool,
}

#[derive(Debug, Clone)]
pub struct Lightstick {
    pub id: String,
    pub group_id: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub release_year: Option<i64>,
    pub image: Option<String>,
    pub verified: bool,
}

pub const SCHEMA: &str = r#"
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
