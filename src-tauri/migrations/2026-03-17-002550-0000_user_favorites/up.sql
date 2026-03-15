-- Your SQL goes here
CREATE TABLE
  IF NOT EXISTS user_favorites_groups (
    group_id TEXT PRIMARY KEY REFERENCES groups (id) ON DELETE CASCADE
  );

-- Favoris artistes solo (pointe sur artists, pas sur groups)
CREATE TABLE
  IF NOT EXISTS user_favorites_artists (
    artist_id TEXT PRIMARY KEY REFERENCES artists (id) ON DELETE CASCADE
  );