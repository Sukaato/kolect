-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_group_members_artist;

DROP INDEX IF EXISTS idx_group_members_group;

DROP TABLE IF EXISTS group_members;