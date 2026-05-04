use crate::models::character;
use crate::net::error::NetworkError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::check_char_name;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::packet::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct CheckCharNameHandler;

impl CheckCharNameHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        _session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let read = check_char_name::read::read_check_char_name_packet(packet)?;
        let exists = character::query::get_character_by_name(state.clone(), &read.ign)
            .await
            .is_ok();
        let result = complete_check_char_name_handler(&read.ign, &exists)?;
        Ok(result)
    }
}

fn complete_check_char_name_handler(
    ign: &str,
    exists: &bool,
) -> Result<HandlerResult<LoginAction>, NetworkError> {
    let mut result: HandlerResult<LoginAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_check_char_name_handler_packet(exists, ign)?
        .finish();
    result.add_action(LoginAction::SendPacket { packet });
    Ok(result)
}
