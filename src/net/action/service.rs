use crate::net::error::NetworkError;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub async fn get_relevant_sessions(
    state: &SharedState,
    session: &Session,
) -> Result<Vec<Session>, NetworkError> {
    let world_id = session
        .world_id
        .ok_or(SessionError::MissingField(session.id))?;
    let channel_id = session
        .channel_id
        .ok_or(SessionError::MissingField(session.id))?;
    let map_id = session
        .map_id
        .ok_or(SessionError::MissingField(session.id))?;
    let mut sessions: Vec<Session> = Vec::new();
    let state = state.lock().await;
    let session_ids = state
        .map_index
        .get(&(world_id, channel_id, map_id))
        .ok_or(SessionError::NoSessions)?;
    for session_id in session_ids {
        sessions.push(
            state
                .sessions
                .get(&session_id)
                .ok_or(SessionError::NotFound(*session_id))?,
        )
    }
    Ok(sessions)
}
