-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_uc_photocard;

DROP INDEX IF EXISTS idx_uc_fanclub_kit;

DROP INDEX IF EXISTS idx_uc_lightstick;

DROP INDEX IF EXISTS idx_uc_digipack;

DROP INDEX IF EXISTS idx_uc_album_version;

DROP INDEX IF EXISTS idx_uc_album;

DROP TABLE IF EXISTS user_collection;