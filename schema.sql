PRAGMA foreign_keys = ON;

-- Groups table
CREATE TABLE
  IF NOT EXISTS groups (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    agency TEXT,
    debut_year INTEGER,
    is_active BOOLEAN
  );

-- Albums table
CREATE TABLE
  IF NOT EXISTS albums (
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

-- Lightsticks table
CREATE TABLE
  IF NOT EXISTS lightsticks (
    id TEXT PRIMARY KEY,
    group_id TEXT NOT NULL,
    name TEXT,
    version TEXT,
    release_year INTEGER,
    image TEXT,
    verified BOOLEAN,
    FOREIGN KEY (group_id) REFERENCES groups (id) ON DELETE CASCADE
  );

-- User collection table
CREATE TABLE
  IF NOT EXISTS user_collection (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    product_type TEXT CHECK (product_type IN ('ALBUM', 'LIGHTSTICK')),
    added_at INTEGER NOT NULL
  );

-- Indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_albums_group ON albums (group_id);

CREATE INDEX IF NOT EXISTS idx_lightsticks_group ON lightsticks (group_id);

CREATE INDEX IF NOT EXISTS idx_collection_product ON user_collection (product_id);