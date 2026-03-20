-- Your SQL goes here
-- Peuplé et rebuilté par le seeder à chaque sync.
CREATE VIRTUAL TABLE IF NOT EXISTS groups_fts USING fts5 (group_id UNINDEXED, name, tokenize = 'unicode61');