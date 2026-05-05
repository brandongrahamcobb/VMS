use crate::net::action;
use crate::net::error::NetworkError;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use tokio::sync::mpsc::UnboundedSender;

pub async fn transmit_action(
    state: &SharedState,
    session: &Session,
    tx: &UnboundedSender<Packet>,
) -> Result<(), NetworkError> {
    let sessions: Vec<Session> = action::service::get_relevant_sessions(state, session).await?;
    for session in sessions {
        let char_id = session
            .char_id
            .ok_or(SessionError::MissingField(session.id))?;
        let packet = Packet::new_empty()
            .build_despawn_player_handler_packet(&char_id)?
            .finish();
        tx.send(packet)?;
    }
    Ok(())
}
