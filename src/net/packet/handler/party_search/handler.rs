use crate::net::action::model::PlayerAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct PartySearchHandler;

impl PartySearchHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        _state: &SharedState,
        _session: &Session,
        _packet: &Packet,
    ) -> Result<HandlerResult<PlayerAction>, NetworkError> {
        let mut result: HandlerResult<PlayerAction> = HandlerResult::new();
        let action = PlayerAction::Simple;
        result.add_action(action);
        Ok(result)
    }
}
