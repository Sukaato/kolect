-- Your SQL goes here
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS collectibles (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK(kind IN ('album', 'lightstick', 'merch', 'photocard')),
    release_date DATE NULL,
    version TEXT NULL,
    barcode TEXT NULL,
    image TEXT NULL,
    verified BOOLEAN NOT NULL,
    group_id TEXT NOT NULL,

    FOREIGN KEY(group_id)
        REFERENCES groups(id)
        ON DELETE CASCADE
);
