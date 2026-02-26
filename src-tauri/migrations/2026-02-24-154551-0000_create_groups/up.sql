CREATE TABLE
  IF NOT EXISTS groups (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    agency TEXT,
    debut_year INTEGER,
    is_active BOOLEAN NOT NULL DEFAULT FALSE
  );