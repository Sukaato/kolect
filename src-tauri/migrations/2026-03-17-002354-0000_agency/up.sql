-- Your SQL goes here
CREATE TABLE
  IF NOT EXISTS agencies (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    country TEXT NOT NULL,
    image_url TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0
  );