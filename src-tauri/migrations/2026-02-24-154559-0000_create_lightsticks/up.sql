CREATE TABLE
  IF NOT EXISTS lightsticks (
    id TEXT PRIMARY KEY,
    group_id TEXT NOT NULL,
    name TEXT,
    version TEXT,
    release_year INTEGER,
    image TEXT,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (group_id) REFERENCES groups (id) ON DELETE CASCADE
  );

CREATE INDEX IF NOT EXISTS idx_lightsticks_group ON lightsticks (group_id);