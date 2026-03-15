-- Your SQL goes here
CREATE TABLE
  IF NOT EXISTS artists (
    id TEXT PRIMARY KEY NOT NULL,
    real_name TEXT NOT NULL,
    birth_date TEXT,
    image_url TEXT,
    solo_debut_date TEXT,
    solo_agency_id TEXT REFERENCES agencies (id) ON DELETE RESTRICT, -- null si l'artiste n'a pas de carrière solo
    is_deleted INTEGER NOT NULL DEFAULT 0
  );

CREATE INDEX idx_artists_solo_agency ON artists (solo_agency_id)
WHERE
  solo_agency_id IS NOT NULL;