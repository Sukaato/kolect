-- Your SQL goes here
-- Exactement un de group_id / artist_id doit être renseigné
-- (fanclub kit de groupe OU fanclub kit d'artiste solo — contrainte applicative)
CREATE TABLE
  IF NOT EXISTS fanclub_kits (
    id TEXT PRIMARY KEY NOT NULL,
    group_id TEXT REFERENCES groups (id) ON DELETE RESTRICT,
    artist_id TEXT REFERENCES artists (id) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    release_date TEXT NOT NULL,
    region TEXT NOT NULL,
    image_url TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0
  );

CREATE INDEX idx_fanclub_kits_group ON fanclub_kits (group_id)
WHERE
  group_id IS NOT NULL;

CREATE INDEX idx_fanclub_kits_artist ON fanclub_kits (artist_id)
WHERE
  artist_id IS NOT NULL;