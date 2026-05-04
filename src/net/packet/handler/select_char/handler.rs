use crate::config::settings;
use crate::inc::helpers;
use crate::models::channel;
use crate::models::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::select_char;
use crate::net::packet::packet::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct SelectCharHandler;

impl SelectCharHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let read = select_char::read::read_select_char_packet(packet)?;
        {
            let state = state.lock().await;
            state
                .sessions
                .update(session.id, |s| s.char_id = Some(read.char_id));
        }
        let addr = settings::get_address()?;
        let octets = helpers::convert_to_ip_array(addr);
        let channel_id = session
            .channel_id
            .ok_or(SessionError::NoChannelSelected(session.id))?;
        let world_id = session
            .world_id
            .ok_or(SessionError::NoWorldSelected(session.id))?;
        let channel = channel::service::resolve_channel(&channel_id, &world_id)?;
        let result = complete_select_char_handler(&read.char_id, &octets, &channel)?;
        Ok(result)
    }
}

fn complete_select_char_handler(
    char_id: &i32,
    octets: &[u8; 4],
    channel: &Channel,
) -> Result<HandlerResult<LoginAction>, NetworkError> {
    let mut result: HandlerResult<LoginAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_select_char_handler_packet(char_id, octets, &channel.port)?
        .finish();
    result.add_action(LoginAction::SendPacket { packet: packet.clone() });
    result.add_action(LoginAction::CloseConnection);
    Ok(result)
}
