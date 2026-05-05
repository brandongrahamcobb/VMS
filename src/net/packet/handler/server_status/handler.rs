use crate::models::world;
use crate::net::action::model::LoginAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct ServerStatusHandler;

impl ServerStatusHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        _state: &SharedState,
        _session: &Session,
        _packet: &Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let worlds = world::service::load_worlds()?;
        let status: i8 = if worlds.iter().any(|world| !world.channels.is_empty()) {
            0
        } else {
            2
        };
        let result = complete_server_status(&status)?;
        Ok(result)
    }
}

fn complete_server_status(status: &i8) -> Result<HandlerResult<LoginAction>, NetworkError> {
    let mut result: HandlerResult<LoginAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_server_status_handler_packet(status)?
        .finish();
    let action = LoginAction::SendLocalPacket {
        packet: packet.clone(),
    };
    result.add_action(action);
    Ok(result)
}
