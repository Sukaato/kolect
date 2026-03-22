-- Your SQL goes here
-- Exactement un de group_id / artist_id doit être renseigné
-- (lightstick de groupe OU lightstick d'artiste solo — contrainte applicative)
-- Ex: lightstick BLACKPINK → group_id, lightstick Lisa → artist_id
CREATE TABLE
  IF NOT EXISTS lightsticks (
    id TEXT PRIMARY KEY NOT NULL,
    group_id TEXT REFERENCES groups (id) ON DELETE RESTRICT,
    artist_id TEXT REFERENCES artists (id) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    release_date TEXT NOT NULL,
    region TEXT NOT NULL,
    image_url TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0
  );

CREATE INDEX idx_lightsticks_group ON lightsticks (group_id)
WHERE
  group_id IS NOT NULL;

CREATE INDEX idx_lightsticks_artist ON lightsticks (artist_id)
WHERE
  artist_id IS NOT NULL;