use crate::db::error::DatabaseError;
use crate::models::account;
use crate::models::character::error::CharacterError;
use crate::models::error::ModelError;
use crate::net::error::NetworkError;
use crate::net::packet::handler::action::ChannelAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::packet::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

const MOVEMENT_HEADER_LEN: usize = 9;

pub struct MovePlayerHandler;

impl MovePlayerHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<ChannelAction>, NetworkError> {
        let acc_id = session
            .acc_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let acc = account::query::get_account_by_id(state.clone(), acc_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let char_id = acc
            .selected_char_id
            .ok_or(CharacterError::NotSelected(acc_id))
            .map_err(ModelError::from)
            .map_err(NetworkError::from)?;
        let mut result: HandlerResult<ChannelAction> = HandlerResult::new();
        if packet.bytes.len() <= 2 + MOVEMENT_HEADER_LEN {
            let action = ChannelAction::Simple;
            result.add_action(action)?;
            return Ok(result);
        }
        let movement_fragment = &packet.bytes[(2 + MOVEMENT_HEADER_LEN)..];
        if movement_fragment.is_empty() || movement_fragment[0] == 0 {
            let action = ChannelAction::Simple;
            result.add_action(action)?;
            return Ok(result);
        }
        let movement_bytes = movement_fragment.to_vec();
        result.add_action(ChannelAction::FieldMove {
            movement_bytes: movement_bytes.clone(),
        })?;
        let packet: Packet = Packet::new_empty()
            .build_player_move_handler_packet(char_id, &movement_bytes)?
            .finish();
        result.add_action(ChannelAction::SendPacket { packet })?;
        Ok(result)
    }
}
