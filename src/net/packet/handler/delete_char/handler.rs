use crate::models::{account, character};
use crate::net::action::model::LoginAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::delete_char;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct DeleteCharHandler;

impl DeleteCharHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let read = delete_char::read::read_delete_char_packet(packet)?;
        let acc_id = session.acc_id;
        let acc = account::query::get_account_by_id(state, &acc_id).await?;
        let status = delete_char::service::check_pic(&acc, &read.pic)?;
        let result = complete_delete_char_handler(state, &acc_id, &read.char_id, &status).await?;
        Ok(result)
    }
}

async fn complete_delete_char_handler(
    state: &SharedState,
    acc_id: &i32,
    char_id: &i32,
    status: &bool,
) -> Result<HandlerResult<LoginAction>, NetworkError> {
    let mut result: HandlerResult<LoginAction> = HandlerResult::new();
    character::query::delete_character(state, acc_id, char_id).await?;
    let packet = Packet::new_empty()
        .build_delete_char_handler_packet(char_id, status)?
        .finish();
    result.add_action(LoginAction::SendLocalPacket {packet: packet.clone()});
    Ok(result)
}
