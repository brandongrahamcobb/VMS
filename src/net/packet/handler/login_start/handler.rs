use crate::net::action::model::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::state::SharedState;

pub struct LoginStartHandler;

impl LoginStartHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        _state: &SharedState,
        _session: &Session,
        _packet: &Packet,
    ) -> Result<HandlerResult<Action>, NetworkError> {
        let mut result: HandlerResult<Action> = HandlerResult::new();
        let action = LoginAction::Simple;
        result.add_action(action);
        Ok(result)
    }
}
