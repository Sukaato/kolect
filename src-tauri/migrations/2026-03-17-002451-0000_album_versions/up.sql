-- Your SQL goes here
-- format enum :
--   'cd'               → CD standard
--   'ep'               → Extended Play
--   'album'            → Album complet
--   'mini_album'       → Mini album
--   'vinyl'            → Vinyle (sorti ultérieurement en général)
--   'kit'              → Kit album (format physique sans CD, ex: BTS)
CREATE TABLE
  IF NOT EXISTS album_versions (
    id TEXT PRIMARY KEY NOT NULL,
    album_id TEXT NOT NULL REFERENCES albums (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    format TEXT NOT NULL CHECK (
      format IN ('cd', 'ep', 'album', 'mini_album', 'vinyl', 'kit')
    ),
    release_date TEXT NOT NULL,
    region TEXT NOT NULL,
    image_url TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0
  );

CREATE INDEX idx_album_versions_album ON album_versions (album_id);