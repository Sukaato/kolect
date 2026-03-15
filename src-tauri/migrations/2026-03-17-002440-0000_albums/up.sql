-- Your SQL goes here
-- Exactement un de group_id / artist_id doit être renseigné
-- (contrainte applicative côté repository)
CREATE TABLE
  IF NOT EXISTS albums (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    release_date TEXT NOT NULL,
    region TEXT NOT NULL,
    image_url TEXT,
    group_id TEXT REFERENCES groups (id) ON DELETE RESTRICT,
    artist_id TEXT REFERENCES artists (id) ON DELETE RESTRICT,
    is_deleted INTEGER NOT NULL DEFAULT 0
  );

CREATE INDEX idx_albums_group ON albums (group_id)
WHERE
  group_id IS NOT NULL;

CREATE INDEX idx_albums_artist ON albums (artist_id)
WHERE
  artist_id IS NOT NULL;