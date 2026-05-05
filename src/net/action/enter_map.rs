use crate::models::character;
use crate::models::character::equipment_set;
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
        let char = character::query::get_character_by_id(state, &char_id).await?;
        let regular_equips =
            equipment_set::query::get_regular_equipment_set_by_character_id(state, &char_id)
                .await?;
        let cash_equips =
            equipment_set::query::get_cash_equipment_set_by_character_id(state, &char_id).await?;
        let packet = Packet::new_empty()
            .build_spawn_player_packet(state, &char, &regular_equips, &cash_equips)
            .await?
            .finish();
        tx.send(packet)?;
    }
    Ok(())
}
