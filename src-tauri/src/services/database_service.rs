// src-tauri/src/services/database_service.rs

use std::fs;
use std::path::Path;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tauri::{AppHandle, Manager};
use tauri_plugin_log::log;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Initialise le fichier de base de données s'il n'existe pas.
pub fn init(app: &AppHandle) {
    if !db_file_exists(app) {
        log::info!("Database file not found, creating it");
        create_db_file(app);
    }
}

/// Applique les migrations en attente.
pub fn migrate(conn: &mut SqliteConnection) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");
}

/// Établit une connexion SQLite vers le fichier de base de données.
pub fn establish_db_connection(app: &AppHandle) -> SqliteConnection {
    let db_path = get_db_path(app);
    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

// ─── Helpers privés ───────────────────────────────────────────────────────────

fn get_db_path(app: &AppHandle) -> String {
    app.path()
        .app_data_dir()
        .expect("Failed to resolve app data directory")
        .join("database.db")
        .to_str()
        .expect("Database path contains invalid UTF-8")
        .to_string()
}

fn db_file_exists(app: &AppHandle) -> bool {
    Path::new(&get_db_path(app)).exists()
}

fn create_db_file(app: &AppHandle) {
    let db_path = get_db_path(app);
    let db_dir = Path::new(&db_path)
        .parent()
        .expect("Failed to get database directory");

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).expect("Failed to create database directory");
    }

    fs::File::create(&db_path).expect("Failed to create database file");
}
