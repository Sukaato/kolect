-- Your SQL goes here
-- kind enum :
--   'group_stage' → nom de scène dans le groupe  (ex: Jimin)
--   'solo_stage'  → nom de scène en solo         (ex: JIMIN)
--   'original'    → nom dans la langue d'origine  (ex: 지민)
--
-- real_name est dans la table artists, pas ici.
-- is_primary : nom à afficher par défaut dans l'UI pour ce kind.
CREATE TABLE
  IF NOT EXISTS artist_aliases (
    id TEXT PRIMARY KEY NOT NULL,
    artist_id TEXT NOT NULL REFERENCES artists (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('group_stage', 'solo_stage', 'original')),
    is_primary INTEGER NOT NULL DEFAULT 0, -- BOOLEAN
    is_deleted INTEGER NOT NULL DEFAULT 0
  );

CREATE INDEX idx_aliases_artist ON artist_aliases (artist_id);