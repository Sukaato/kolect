-- Your SQL goes here
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS groups (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    agency TEXT NOT NULL,
    debut_year INTEGER NOT NULL,
    is_active BOOLEAN NOT NULL
);