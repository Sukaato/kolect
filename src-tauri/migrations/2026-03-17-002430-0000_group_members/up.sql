-- Your SQL goes here
-- roles  : valeurs séparées par virgule ex: 'vocalist,dancer,rapper'
-- leave_date null  → membre encore actif dans le groupe
-- leave_date renseignée → ancien membre
CREATE TABLE
  IF NOT EXISTS group_members (
    artist_id TEXT NOT NULL REFERENCES artists (id) ON DELETE CASCADE,
    group_id TEXT NOT NULL REFERENCES groups (id) ON DELETE CASCADE,
    roles TEXT NOT NULL DEFAULT '',
    join_date TEXT,
    leave_date TEXT,
    PRIMARY KEY (artist_id, group_id)
  );

CREATE INDEX idx_group_members_group ON group_members (group_id);

CREATE INDEX idx_group_members_artist ON group_members (artist_id);