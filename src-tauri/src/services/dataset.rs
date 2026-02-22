use crate::db::DbLocation;
use rusqlite::{params, Connection, OptionalExtension};
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lightstick {
    pub id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(rename = "datasetVersion")]
    pub dataset_version: String,
    #[serde(rename = "generatedAt")]
    pub generated_at: String,
    #[serde(default)]
    pub groups: Vec<Group>,
    #[serde(default)]
    pub albums: Vec<Album>,
    #[serde(default)]
    pub lightsticks: Vec<Lightstick>,
}

async fn fetch_from_remote(url: &str) -> Result<Dataset, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch dataset: {}", e))?;

    response
        .json::<Dataset>()
        .await
        .map_err(|e| format!("Failed to parse dataset: {}", e))
}

fn ensure_tables(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS dataset_metadata (
            version TEXT NOT NULL,
            generated_at TEXT NOT NULL,
            fetched_at INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS groups (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS albums (
            id TEXT PRIMARY KEY,
            group_id TEXT NOT NULL,
            title TEXT NOT NULL,
            FOREIGN KEY(group_id) REFERENCES groups(id)
        );
        CREATE TABLE IF NOT EXISTS lightsticks (
            id TEXT PRIMARY KEY,
            group_id TEXT NOT NULL,
            name TEXT NOT NULL,
            FOREIGN KEY(group_id) REFERENCES groups(id)
        );",
    )
    .map_err(|e| format!("Failed to create dataset tables: {}", e))
}

fn save_dataset(dataset: &Dataset) -> Result<(), String> {
    let loc = DbLocation::Default;
    let mut conn = loc.connect().map_err(|e| e.to_string())?;

    ensure_tables(&conn)?;

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Clear existing data
    tx.execute("DELETE FROM dataset_metadata", [])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM lightsticks", [])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM albums", [])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM groups", [])
        .map_err(|e| e.to_string())?;

    // Insert metadata
    let fetched_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    tx.execute(
        "INSERT INTO dataset_metadata (version, generated_at, fetched_at) VALUES (?1, ?2, ?3)",
        params![dataset.dataset_version, dataset.generated_at, fetched_at],
    )
    .map_err(|e| e.to_string())?;

    // Insert groups
    {
        let mut stmt = tx
            .prepare("INSERT INTO groups (id, name) VALUES (?1, ?2)")
            .map_err(|e| e.to_string())?;
        for group in &dataset.groups {
            stmt.execute(params![group.id, group.name])
                .map_err(|e| e.to_string())?;
        }
    }

    // Insert albums
    {
        let mut stmt = tx
            .prepare("INSERT INTO albums (id, group_id, title) VALUES (?1, ?2, ?3)")
            .map_err(|e| e.to_string())?;
        for album in &dataset.albums {
            stmt.execute(params![album.id, album.group_id, album.title])
                .map_err(|e| e.to_string())?;
        }
    }

    // Insert lightsticks
    {
        let mut stmt = tx
            .prepare("INSERT INTO lightsticks (id, group_id, name) VALUES (?1, ?2, ?3)")
            .map_err(|e| e.to_string())?;
        for lightstick in &dataset.lightsticks {
            stmt.execute(params![lightstick.id, lightstick.group_id, lightstick.name])
                .map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn get() -> Result<Dataset, String> {
    let loc = DbLocation::Default;
    let conn = loc.connect().map_err(|e| e.to_string())?;

    ensure_tables(&conn)?;

    let metadata = conn
        .query_row(
            "SELECT version, generated_at FROM dataset_metadata LIMIT 1",
            [],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|e| e.to_string())?;

    let (version, generated_at) = match metadata {
        Some(v) => v,
        None => return Err("Dataset not found".to_string()),
    };

    // Fetch groups
    let mut stmt = conn
        .prepare("SELECT id, name FROM groups")
        .map_err(|e| e.to_string())?;
    let groups = stmt
        .query_map([], |row| {
            Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // Fetch albums
    let mut stmt = conn
        .prepare("SELECT id, group_id, title FROM albums")
        .map_err(|e| e.to_string())?;
    let albums = stmt
        .query_map([], |row| {
            Ok(Album {
                id: row.get(0)?,
                group_id: row.get(1)?,
                title: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // Fetch lightsticks
    let mut stmt = conn
        .prepare("SELECT id, group_id, name FROM lightsticks")
        .map_err(|e| e.to_string())?;
    let lightsticks = stmt
        .query_map([], |row| {
            Ok(Lightstick {
                id: row.get(0)?,
                group_id: row.get(1)?,
                name: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(Dataset {
        dataset_version: version,
        generated_at,
        groups,
        albums,
        lightsticks,
    })
}

/// Sync dataset from remote URL
pub async fn sync(url: &str) -> Result<bool, String> {
    let remote_dataset = fetch_from_remote(url).await?;

    let loc = DbLocation::Default;
    let conn = loc.connect().map_err(|e| e.to_string())?;
    ensure_tables(&conn)?;

    let local_version: Option<String> = conn
        .query_row("SELECT version FROM dataset_metadata LIMIT 1", [], |row| {
            row.get(0)
        })
        .optional()
        .map_err(|e| e.to_string())?;

    let needs_update = match local_version {
        Some(v) => {
            let local = Version::parse(&v);
            let remote = Version::parse(&remote_dataset.dataset_version);

            match (local, remote) {
                (Ok(l), Ok(r)) => r > l,
                _ => remote_dataset.dataset_version != v,
            }
        }
        None => true,
    };

    if needs_update {
        save_dataset(&remote_dataset)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
