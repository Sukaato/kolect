-- Your SQL goes here
CREATE TABLE
  IF NOT EXISTS groups (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    debut_date TEXT NOT NULL,
    fandom_name TEXT,
    image_url TEXT,
    agency_id TEXT NOT NULL REFERENCES agencies (id) ON DELETE RESTRICT,
    is_deleted INTEGER NOT NULL DEFAULT 0
  );

CREATE INDEX idx_groups_agency ON groups (agency_id);