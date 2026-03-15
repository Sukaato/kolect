-- Your SQL goes here
-- Exactement un de album_id / album_version_id / digipack_id
-- doit être renseigné (contrainte applicative) :
--   album_id seul          → photocard random (toutes versions)
--   album_version_id seul  → photocard d'une version spécifique
--   digipack_id seul       → photocard incluse dans un digipack
CREATE TABLE
  IF NOT EXISTS photocards (
    id TEXT PRIMARY KEY NOT NULL,
    artist_id TEXT REFERENCES artists (id) ON DELETE RESTRICT,
    album_id TEXT REFERENCES albums (id) ON DELETE CASCADE,
    album_version_id TEXT REFERENCES album_versions (id) ON DELETE CASCADE,
    digipack_id TEXT REFERENCES digipacks (id) ON DELETE CASCADE,
    release_date TEXT NOT NULL,
    region TEXT NOT NULL,
    image_url TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0
  );

CREATE INDEX idx_photocards_artist ON photocards (artist_id);

CREATE INDEX idx_photocards_album ON photocards (album_id)
WHERE
  album_id IS NOT NULL;

CREATE INDEX idx_photocards_version ON photocards (album_version_id)
WHERE
  album_version_id IS NOT NULL;

CREATE INDEX idx_photocards_digipack ON photocards (digipack_id)
WHERE
  digipack_id IS NOT NULL;