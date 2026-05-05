use crate::models::channel;
use crate::models::channel::model::Channel;
use crate::net::action::model::PlayerAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::cc;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct ChangeChannelHandler;

impl ChangeChannelHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        _state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult<PlayerAction>, NetworkError> {
        let read = cc::read::read_change_channel_packet(packet)?;
        let world_id = session
            .world_id
            .ok_or(SessionError::NoWorldSelected(session.id))?;
        let channel = channel::service::resolve_channel(&read.channel_id, &world_id)?;
        let char_id = session.char_id.ok_or(SessionError::NotFound(session.id))?;
        let result = complete_change_channel_handler(session, &char_id, &channel)?;
        Ok(result)
    }
}

fn complete_change_channel_handler(
    session: &Session,
    char_id: &i32,
    channel: &Channel,
) -> Result<HandlerResult<PlayerAction>, NetworkError> {
    let mut result: HandlerResult<PlayerAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_channel_change_handler_packet(channel)?
        .finish();
    result.add_action(PlayerAction::SendLocalPacket {packet: packet.clone()});
    let packet: Packet = Packet::new_empty()
        .build_despawn_player_handler_packet(char_id)?
        .finish();
    result.add_action(PlayerAction::EnterMap {
        session: session.clone(),
        packet,
    });
    Ok(result)
}
