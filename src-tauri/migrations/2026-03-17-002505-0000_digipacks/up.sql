-- Your SQL goes here
-- Un digipack est lié à un album ET à un artiste spécifique
-- (ex: digipack Lisa de Born Pink → album_id=born_pink, artist_id=lisa)
CREATE TABLE
  IF NOT EXISTS digipacks (
    id TEXT PRIMARY KEY NOT NULL,
    album_id TEXT NOT NULL REFERENCES albums (id) ON DELETE CASCADE,
    artist_id TEXT REFERENCES artists (id) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    release_date TEXT NOT NULL,
    region TEXT NOT NULL,
    image_url TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0
  );

CREATE INDEX idx_digipacks_album ON digipacks (album_id);

CREATE INDEX idx_digipacks_artist ON digipacks (artist_id);