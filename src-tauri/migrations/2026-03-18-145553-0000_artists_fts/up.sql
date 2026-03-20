-- Your SQL goes here
-- Le champ name agrège real_name + tous les aliases.
-- Peuplé et rebuilté par le seeder à chaque sync.
CREATE VIRTUAL TABLE IF NOT EXISTS artists_fts USING fts5 (artist_id UNINDEXED, name, tokenize = 'unicode61');