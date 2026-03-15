-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_fanclub_kits_artist;

DROP INDEX IF EXISTS idx_fanclub_kits_group;

DROP TABLE IF EXISTS fanclub_kits;