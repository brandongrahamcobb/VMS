use crate::models::error::ModelError;
use crate::models::world;
use crate::net::error::NetworkError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::packet::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct WorldListHandler;

impl WorldListHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        _state: SharedState,
        _session: Session,
        _packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let worlds = world::service::load_worlds()
            .map_err(ModelError::from)
            .map_err(NetworkError::from)?;
        let mut result: HandlerResult<LoginAction> = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_list_worlds_handler_servers_packet(worlds)?
            .finish();
        let action = LoginAction::SendPacket { packet };
        result.add_action(action)?;
        let packet: Packet = Packet::new_empty()
            .build_list_worlds_handler_last_connected_world_packet()?
            .finish();
        let action = LoginAction::SendPacket { packet };
        result.add_action(action)?;
        let packet: Packet = Packet::new_empty()
            .build_list_worlds_handler_recommended_worlds_packet()?
            .finish();
        let action = LoginAction::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}
