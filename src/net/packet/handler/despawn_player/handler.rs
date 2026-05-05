use crate::net::action::model::PlayerAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::despawn_player;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct DespawnPlayerHandler;

impl DespawnPlayerHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        _state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult<PlayerAction>, NetworkError> {
        let read = despawn_player::read::read_despawn_player_handler_packet(packet)?;
        let mut result = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_handler_packet(&read.char_id)?
            .finish();
        result.add_action(PlayerAction::SendLocalPacket {
            packet: packet.clone(),
        });
        result.add_action(PlayerAction::ExitMap {
            session: session.clone(),
            packet: packet.clone(),
        });
        Ok(result)
    }
}
