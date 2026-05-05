use crate::net::action::model::PlayerAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
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
        _state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult<PlayerAction>, NetworkError> {
        let char_id = session
            .char_id
            .ok_or(SessionError::NoCharacterSelected(session.id))?;
        let mut result: HandlerResult<PlayerAction> = HandlerResult::new();
        if packet.bytes.len() <= 2 + MOVEMENT_HEADER_LEN {
            let action = PlayerAction::Simple;
            result.add_action(action);
            return Ok(result);
        }
        let movement_fragment = &packet.bytes[(2 + MOVEMENT_HEADER_LEN)..];
        if movement_fragment.is_empty() || movement_fragment[0] == 0 {
            let action = PlayerAction::Simple;
            result.add_action(action);
            return Ok(result);
        }
        let movement_bytes = movement_fragment.to_vec();
        let packet: Packet = Packet::new_empty()
            .build_player_move_handler_packet(&char_id, &movement_bytes)?
            .finish();
        result.add_action(PlayerAction::SendLocalPacket {
            packet: packet.clone(),
        });
        result.add_action(PlayerAction::FieldMove {
            session: session.clone(),
            packet: packet.clone(),
        });
        Ok(result)
    }
}
