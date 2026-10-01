use tauri::State;
use crate::database::SessionManager;
use crate::models::TimerSession;

#[tauri::command]
pub async fn test_db(manager: State<'_, SessionManager>) -> Result<String, String> {
    // Create a test session
    let test_session = TimerSession {
        id: manager.generate_id(),
        start_time: "2024-01-01T10:00:00+00:00".to_string(),
        end_time: Some("2024-01-01T11:00:00+00:00".to_string()),
        duration: 3600,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    manager.add_session(test_session.clone()).await?;

    // Verify it was saved
    let sessions = manager.get_all_sessions().await?;

    Ok(format!("Test complete. Sessions in DB: {}", sessions.len()))
}
