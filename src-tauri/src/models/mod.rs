use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SessionData {
    pub session: TimerSession,
    pub breaks: Vec<Break>,
}

impl Serialize for SessionData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("SessionData", 2)?;
        state.serialize_field("session", &self.session)?;
        state.serialize_field("breaks", &self.breaks)?;
        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimerSession {
    pub id: i64,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration: i64, // in seconds (calculated: end_time - start_time - breaks)
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Break {
    pub id: i64,
    pub session_id: i64,
    pub start_time: String,
    pub end_time: String,
    pub duration: i64, // in seconds
}
