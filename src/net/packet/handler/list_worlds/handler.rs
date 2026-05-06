use crate::models::world;
use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::list_worlds::reader::ListWorldsReader;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct ListWorldsHandler;

impl ListWorldsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let reader: ListWorldsReader = ListWorldsReader::new().read_list_worlds_packet(packet)?;
        let store: ListWorldsStore =
            ListWorldsStore::new().store_list_worlds(state, session, &reader)?;
        let result: HandlerResult = self.build_list_worlds_result(&store)?;
        Ok(result)
    }

    fn build_list_worlds_result(
        &self,
        store: &ListWorldsStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_list_worlds_handler_servers_packet(&store.worlds)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        let packet: Packet = Packet::new_empty()
            .build_list_worlds_handler_last_connected_world_packet()?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        let packet: Packet = Packet::new_empty()
            .build_list_worlds_handler_recommended_worlds_packet()?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
    }
}
