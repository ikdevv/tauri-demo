use tauri::State;
use crate::database::SessionManager;
use crate::models::{TimerSession, SessionData};
use crate::services;

#[tauri::command]
pub async fn save_timer_session(
    #[allow(non_snake_case)]
    startTime: String,
    duration: i64,
    manager: State<'_, SessionManager>,
) -> Result<i64, String> {
    services::create_timer_session(startTime, duration, &manager).await
}

#[tauri::command]
pub async fn add_break(
    #[allow(non_snake_case)]
    sessionId: i64,
    #[allow(non_snake_case)]
    startTime: String,
    #[allow(non_snake_case)]
    endTime: String,
    manager: State<'_, SessionManager>,
) -> Result<i64, String> {
    services::add_break(sessionId, startTime, endTime, &manager).await
}

#[tauri::command]
pub async fn get_timer_sessions(
    manager: State<'_, SessionManager>,
) -> Result<Vec<TimerSession>, String> {
    services::fetch_timer_sessions(&manager).await
}

#[tauri::command]
pub async fn get_sessions_with_breaks(
    manager: State<'_, SessionManager>,
) -> Result<Vec<SessionData>, String> {
    services::fetch_timer_sessions_with_breaks(&manager).await
}
