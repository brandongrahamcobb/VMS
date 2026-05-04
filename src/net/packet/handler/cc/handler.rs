use crate::models::channel;
use crate::models::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::action::ChannelAction;
use crate::net::packet::handler::cc;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::packet::Packet;
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
        _state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<ChannelAction>, NetworkError> {
        let read = cc::read::read_change_channel_packet(packet)?;
        let world_id = session
            .world_id
            .ok_or(SessionError::NoWorldSelected(session.id))?;
        let channel = channel::service::resolve_channel(&read.channel_id, &world_id)?;
        let result = complete_change_channel_handler(&channel)?;
        Ok(result)
    }
}

fn complete_change_channel_handler(
    channel: &Channel,
) -> Result<HandlerResult<ChannelAction>, NetworkError> {
    let mut result: HandlerResult<ChannelAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_channel_change_handler_packet(channel)?
        .finish();
    result.add_action(ChannelAction::SendPacket { packet });
    Ok(result)
}
