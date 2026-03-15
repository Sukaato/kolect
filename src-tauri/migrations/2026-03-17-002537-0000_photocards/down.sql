-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_photocards_digipack;

DROP INDEX IF EXISTS idx_photocards_version;

DROP INDEX IF EXISTS idx_photocards_album;

DROP INDEX IF EXISTS idx_photocards_artist;

DROP TABLE IF EXISTS photocards;