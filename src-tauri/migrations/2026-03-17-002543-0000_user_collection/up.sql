-- Your SQL goes here
-- Exactement une FK doit être renseignée par ligne.
-- Plusieurs lignes avec la même FK = doublons physiques autorisés.
-- Le comptage owned utilise COUNT(DISTINCT fk) côté repository.
--
-- is_signed : pertinent uniquement pour album_version, digipack et photocard.
-- acquired_at : date d'acquisition au format ISO 8601.
CREATE TABLE
  IF NOT EXISTS user_collection (
    id TEXT PRIMARY KEY NOT NULL,
    album_id TEXT REFERENCES albums (id) ON DELETE CASCADE,
    album_version_id TEXT REFERENCES album_versions (id) ON DELETE CASCADE,
    digipack_id TEXT REFERENCES digipacks (id) ON DELETE CASCADE,
    lightstick_id TEXT REFERENCES lightsticks (id) ON DELETE CASCADE,
    fanclub_kit_id TEXT REFERENCES fanclub_kits (id) ON DELETE CASCADE,
    photocard_id TEXT REFERENCES photocards (id) ON DELETE CASCADE,
    acquired_at TEXT NOT NULL DEFAULT (datetime ('now')),
    notes TEXT,
    is_signed INTEGER NOT NULL DEFAULT 0 -- BOOLEAN
  );

CREATE INDEX idx_uc_album ON user_collection (album_id)
WHERE
  album_id IS NOT NULL;

CREATE INDEX idx_uc_album_version ON user_collection (album_version_id)
WHERE
  album_version_id IS NOT NULL;

CREATE INDEX idx_uc_digipack ON user_collection (digipack_id)
WHERE
  digipack_id IS NOT NULL;

CREATE INDEX idx_uc_lightstick ON user_collection (lightstick_id)
WHERE
  lightstick_id IS NOT NULL;

CREATE INDEX idx_uc_fanclub_kit ON user_collection (fanclub_kit_id)
WHERE
  fanclub_kit_id IS NOT NULL;

CREATE INDEX idx_uc_photocard ON user_collection (photocard_id)
WHERE
  photocard_id IS NOT NULL;