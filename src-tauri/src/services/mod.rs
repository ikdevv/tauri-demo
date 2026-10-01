use chrono::Utc;
use crate::database::SessionManager;
use crate::models::{TimerSession, Break, SessionData};

pub async fn create_timer_session(
    start_time: String,
    duration: i64,
    manager: &SessionManager,
) -> Result<i64, String> {
    let id = manager.generate_id();
    let end_time = Some(Utc::now().to_rfc3339());

    let session = TimerSession {
        id,
        start_time,
        end_time,
        duration,
        created_at: Utc::now().to_rfc3339(),
    };
    manager.add_session(session).await?;
    Ok(id)
}

pub async fn add_break(
    session_id: i64,
    start_time: String,
    end_time: String,
    manager: &SessionManager,
) -> Result<i64, String> {
    let id = manager.generate_id();

    let start = chrono::DateTime::parse_from_rfc3339(&start_time)
        .map(|dt| dt.timestamp())
        .map_err(|e| e.to_string())?;
    let end = chrono::DateTime::parse_from_rfc3339(&end_time)
        .map(|dt| dt.timestamp())
        .map_err(|e| e.to_string())?;
    let duration = end - start;

    let brk = Break {
        id,
        session_id,
        start_time,
        end_time,
        duration,
    };

    manager.add_break(brk).await?;
    Ok(duration)
}

pub async fn fetch_timer_sessions_with_breaks(
    manager: &SessionManager,
) -> Result<Vec<SessionData>, String> {
    let sessions = manager.get_all_sessions().await?;

    let mut result = Vec::new();
    for session in sessions {
        let breaks = manager.get_breaks(session.id).await.unwrap_or_default();
        result.push(SessionData { session, breaks });
    }

    Ok(result)
}

pub async fn fetch_timer_sessions(manager: &SessionManager) -> Result<Vec<TimerSession>, String> {
    manager.get_all_sessions().await
}
