-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_albums_artist;

DROP INDEX IF EXISTS idx_albums_group;

DROP TABLE IF EXISTS albums;