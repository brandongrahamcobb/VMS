use crate::config::settings;
use crate::inc::helpers;
use crate::models::channel;
use crate::models::channel::model::Channel;
use crate::net::action::model::LoginAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::register_pic;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct RegisterPicHandler;

impl RegisterPicHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let read = register_pic::read::read_register_pic_packet(packet)?;
        let channel_id = session
            .channel_id
            .ok_or(SessionError::NoChannelSelected(session.id))?;
        let world_id = session
            .world_id
            .ok_or(SessionError::NoWorldSelected(session.id))?;
        register_pic::service::set_pic(state, &session, &read.pic).await?;
        let addr = settings::get_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(&addr);
        let channel = channel::service::resolve_channel(&channel_id, &world_id)?;
        let result = complete_register_pic_handler(&read.char_id, &octets, &channel)?;
        Ok(result)
    }
}

fn complete_register_pic_handler(
    char_id: &i32,
    octets: &[u8; 4],
    channel: &Channel,
) -> Result<HandlerResult<LoginAction>, NetworkError> {
    let mut result: HandlerResult<LoginAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_select_char_handler_packet(char_id, octets, &channel.port)?
        .finish();
    result.add_action(LoginAction::SendLocalPacket {
        packet: packet.clone(),
    });
    Ok(result)
}
