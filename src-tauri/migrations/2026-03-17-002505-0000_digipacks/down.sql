-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_digipacks_artist;

DROP INDEX IF EXISTS idx_digipacks_album;

DROP TABLE IF EXISTS digipacks;