use crate::models::account;
use crate::models::account::model::Account;
use crate::net::error::NetworkError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::tos;
use crate::net::packet::packet::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct TOSHandler;

impl TOSHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let read = tos::read::read_tos_packet(packet)?;
        if read.confirmed != 0x01 {
            let mut result: HandlerResult<LoginAction> = HandlerResult::new();
            result.add_action(LoginAction::Simple);
            return Ok(result);
        }
        let acc_id = session.acc_id;
        let mut acc = account::query::get_account_by_id(state.clone(), &acc_id).await?;
        acc.accepted_tos = true;
        account::query::update(state.clone(), &acc).await?;
        let result = complete_tos_handler(&acc)?;
        Ok(result)
    }
}

fn complete_tos_handler(acc: &Account) -> Result<HandlerResult<LoginAction>, NetworkError> {
    let mut result: HandlerResult<LoginAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_credentials_handler_successful_login_packet(&acc)?
        .finish();
    let action = LoginAction::SendPacket { packet };
    result.add_action(action);
    Ok(result)
}
