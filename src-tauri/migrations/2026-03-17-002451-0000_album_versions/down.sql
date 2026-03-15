-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_album_versions_album;

DROP TABLE IF EXISTS album_versions;