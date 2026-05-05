use crate::net::action;
use crate::net::error::NetworkError;
use crate::net::packet::model::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub async fn transmit_action(
    state: &SharedState,
    session: &Session,
    packet: &Packet,
) -> Result<(), NetworkError> {
    let sessions: Vec<Session> = action::service::get_relevant_sessions(state, session).await?;
    for target in sessions {
        if target.id == session.id {
            continue;
        }
        target.tx.send(packet.clone())?;
    }
    Ok(())
}
