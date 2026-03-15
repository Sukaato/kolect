-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_aliases_artist;

DROP TABLE IF EXISTS artist_aliases;