-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS uc_after_delete_collection_groups_fts;

DROP TRIGGER IF EXISTS uc_after_insert_collection_groups_fts;

DROP TABLE IF EXISTS collection_groups_fts;