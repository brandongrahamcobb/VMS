use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::server_status::reader::ServerStatusReader;
use crate::net::packet::handler::server_status::store::ServerStatusStore;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ServerStatusHandler;

impl ServerStatusHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: ServerStatusReader = ServerStatusReader::read_server_status_packet(packet)?;
        let store: ServerStatusStore =
            ServerStatusStore::store_server_status(state, session, reader.clone()).await?;
        let result: HandlerResult = self.build_server_status_result(store.clone())?;
        Ok(result)
    }

    fn build_server_status_result(
        &self,
        store: ServerStatusStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_server_status_handler_packet(store.status)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
