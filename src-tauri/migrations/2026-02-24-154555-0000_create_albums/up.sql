CREATE TABLE
  IF NOT EXISTS albums (
    id TEXT PRIMARY KEY,
    group_id TEXT NOT NULL,
    title TEXT NOT NULL,
    kind TEXT CHECK (kind IN ('EP', 'Album', 'Single', 'Mini')),
    release_date TEXT,
    cover_image TEXT,
    barcode TEXT,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (group_id) REFERENCES groups (id) ON DELETE CASCADE
  );

CREATE INDEX IF NOT EXISTS idx_albums_group ON albums (group_id);