use crate::models::{TimerSession, Break};
use rusqlite::{Connection, params};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static DB: Lazy<Mutex<Option<Connection>>> = Lazy::new(|| Mutex::new(None));

fn get_db_path() -> String {
    // Try multiple paths to find the right location
    let candidates = vec![
        "timer.db",
        "../timer.db",
        "../../timer.db",
    ];

    for path in candidates {
        if std::path::Path::new(path).exists() || path == "timer.db" {
            eprintln!("Using database path: {}", path);
            return path.to_string();
        }
    }

    // Fallback to current directory
    "timer.db".to_string()
}

pub struct SessionManager;

impl SessionManager {
    pub fn new() -> Self {
        Self::init_db().ok();
        Self
    }

    fn init_db() -> Result<(), String> {
        let db_path = get_db_path();
        eprintln!("Initializing database at: {}", db_path);

        let conn = Connection::open(&db_path).map_err(|e| {
            eprintln!("Failed to open database: {}", e);
            e.to_string()
        })?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS timer_sessions (
                id INTEGER PRIMARY KEY,
                start_time TEXT NOT NULL,
                end_time TEXT,
                duration INTEGER NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| {
            eprintln!("Failed to create timer_sessions table: {}", e);
            e.to_string()
        })?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS session_breaks (
                id INTEGER PRIMARY KEY,
                session_id INTEGER NOT NULL,
                start_time TEXT NOT NULL,
                end_time TEXT NOT NULL,
                duration INTEGER NOT NULL,
                FOREIGN KEY(session_id) REFERENCES timer_sessions(id)
            )",
            [],
        )
        .map_err(|e| {
            eprintln!("Failed to create session_breaks table: {}", e);
            e.to_string()
        })?;

        let mut db = DB.lock().map_err(|e| {
            eprintln!("Failed to acquire DB lock: {}", e);
            e.to_string()
        })?;
        *db = Some(conn);
        eprintln!("Database initialized successfully");
        Ok(())
    }

    pub async fn add_session(&self, session: TimerSession) -> Result<(), String> {
        eprintln!("Adding session: id={}, start={}", session.id, session.start_time);

        let db = DB.lock().map_err(|e| {
            eprintln!("Failed to acquire DB lock in add_session: {}", e);
            e.to_string()
        })?;

        if let Some(conn) = db.as_ref() {
            conn.execute(
                "INSERT INTO timer_sessions (id, start_time, end_time, duration, created_at) VALUES (?, ?, ?, ?, ?)",
                params![session.id, session.start_time, session.end_time, session.duration, session.created_at],
            )
            .map_err(|e| {
                eprintln!("Failed to insert session: {}", e);
                e.to_string()
            })?;
            eprintln!("Session inserted successfully");
            Ok(())
        } else {
            eprintln!("Database not initialized");
            Err("Database not initialized".to_string())
        }
    }

    pub async fn add_break(&self, brk: Break) -> Result<(), String> {
        eprintln!("Adding break: id={}, session_id={}", brk.id, brk.session_id);

        let db = DB.lock().map_err(|e| {
            eprintln!("Failed to acquire DB lock in add_break: {}", e);
            e.to_string()
        })?;

        if let Some(conn) = db.as_ref() {
            conn.execute(
                "INSERT INTO session_breaks (id, session_id, start_time, end_time, duration) VALUES (?, ?, ?, ?, ?)",
                params![brk.id, brk.session_id, brk.start_time, brk.end_time, brk.duration],
            )
            .map_err(|e| {
                eprintln!("Failed to insert break: {}", e);
                e.to_string()
            })?;
            eprintln!("Break inserted successfully");
            Ok(())
        } else {
            eprintln!("Database not initialized");
            Err("Database not initialized".to_string())
        }
    }

    pub async fn get_all_sessions(&self) -> Result<Vec<TimerSession>, String> {
        eprintln!("Fetching all sessions");

        let db = DB.lock().map_err(|e| {
            eprintln!("Failed to acquire DB lock in get_all_sessions: {}", e);
            e.to_string()
        })?;

        if let Some(conn) = db.as_ref() {
            let mut stmt = conn
                .prepare("SELECT id, start_time, end_time, duration, created_at FROM timer_sessions ORDER BY id DESC")
                .map_err(|e| {
                    eprintln!("Failed to prepare statement: {}", e);
                    e.to_string()
                })?;

            let sessions = stmt
                .query_map([], |row| {
                    Ok(TimerSession {
                        id: row.get(0)?,
                        start_time: row.get(1)?,
                        end_time: row.get(2)?,
                        duration: row.get(3)?,
                        created_at: row.get(4)?,
                    })
                })
                .map_err(|e| {
                    eprintln!("Failed to query sessions: {}", e);
                    e.to_string()
                })?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| {
                    eprintln!("Failed to collect sessions: {}", e);
                    e.to_string()
                })?;

            eprintln!("Fetched {} sessions", sessions.len());
            Ok(sessions)
        } else {
            eprintln!("Database not initialized");
            Err("Database not initialized".to_string())
        }
    }

    pub async fn get_breaks(&self, session_id: i64) -> Result<Vec<Break>, String> {
        eprintln!("Fetching breaks for session: {}", session_id);

        let db = DB.lock().map_err(|e| {
            eprintln!("Failed to acquire DB lock in get_breaks: {}", e);
            e.to_string()
        })?;

        if let Some(conn) = db.as_ref() {
            let mut stmt = conn
                .prepare("SELECT id, session_id, start_time, end_time, duration FROM session_breaks WHERE session_id = ? ORDER BY id ASC")
                .map_err(|e| {
                    eprintln!("Failed to prepare breaks statement: {}", e);
                    e.to_string()
                })?;

            let breaks = stmt
                .query_map(params![session_id], |row| {
                    Ok(Break {
                        id: row.get(0)?,
                        session_id: row.get(1)?,
                        start_time: row.get(2)?,
                        end_time: row.get(3)?,
                        duration: row.get(4)?,
                    })
                })
                .map_err(|e| {
                    eprintln!("Failed to query breaks: {}", e);
                    e.to_string()
                })?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| {
                    eprintln!("Failed to collect breaks: {}", e);
                    e.to_string()
                })?;

            eprintln!("Fetched {} breaks for session {}", breaks.len(), session_id);
            Ok(breaks)
        } else {
            eprintln!("Database not initialized");
            Err("Database not initialized".to_string())
        }
    }

    pub fn generate_id(&self) -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64
    }
}
