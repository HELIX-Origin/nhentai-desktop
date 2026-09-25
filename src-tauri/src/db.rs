use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

pub struct Db {
    conn: Mutex<Connection>,
}

impl Db {
    pub fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS kv (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS api_key (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                key TEXT NOT NULL
            );",
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM kv WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        match rows.next()? {
            Some(row) => row.get(0).map(Some),
            None => Ok(None),
        }
    }

    pub fn set(&self, key: &str, value: &str) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO kv (key, value, updated_at) VALUES (?1, ?2, strftime('%s','now'))
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn del(&self, key: &str) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM kv WHERE key = ?1", params![key])?;
        Ok(())
    }

    pub fn dump(&self) -> Result<Vec<(String, String)>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key, value FROM kv")?;
        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn clear(&self) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM kv", [])?;
        Ok(())
    }

    pub fn set_api_key(&self, key: &str) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO api_key (id, key) VALUES (1, ?1)
             ON CONFLICT(id) DO UPDATE SET key = excluded.key",
            params![key],
        )?;
        Ok(())
    }

    pub fn api_key(&self) -> Result<Option<String>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key FROM api_key WHERE id = 1")?;
        let mut rows = stmt.query([])?;
        match rows.next()? {
            Some(row) => row.get(0).map(Some),
            None => Ok(None),
        }
    }

    pub fn prune_cache(&self, before_ts: i64) -> Result<usize, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let n = conn.execute(
            "DELETE FROM kv WHERE key LIKE 'nh-desktop:cache:%' AND updated_at < ?1",
            params![before_ts],
        )?;
        Ok(n)
    }

    pub fn clear_api_key(&self) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM api_key WHERE id = 1", [])?;
        Ok(())
    }
}