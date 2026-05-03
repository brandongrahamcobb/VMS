use crate::net::error::NetworkError;
use crate::net::packet::packet::Packet;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct LoginStartHandler;

impl LoginStartHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        _state: SharedState,
        _session: Session,
        _packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut result: HandlerResult<LoginAction> = HandlerResult::new();
        let action = LoginAction::Simple;
        result.add_action(action)?;
        Ok(result)
    }
}
