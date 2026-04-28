use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::handler::action::Action;
use crate::net::packet::handler::result::HandlerResult;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct LoginStartHandler;

impl LoginStartHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        _state: SharedState,
        _session: Session,
        _packet: Packet,
    ) -> Result<HandlerResult<Action>, NetworkError> {
        let mut result = HandlerResult::new();
        let action = Action::Simple;
        result.add_action(action)?;
        Ok(result)
    }
}
