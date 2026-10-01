mod commands;
mod database;
mod models;
mod services;

use database::SessionManager;
use commands::{greet, calculate_total, save_timer_session, get_timer_sessions, add_break, get_sessions_with_breaks, test_db};
use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create_timer_sessions_table",
        sql: "CREATE TABLE IF NOT EXISTS timer_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                duration INTEGER NOT NULL,
                created_at TEXT NOT NULL
            );",
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:../timer.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .manage(SessionManager::new())
        .invoke_handler(tauri::generate_handler![
            greet,
            calculate_total,
            save_timer_session,
            get_timer_sessions,
            add_break,
            get_sessions_with_breaks,
            test_db
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
