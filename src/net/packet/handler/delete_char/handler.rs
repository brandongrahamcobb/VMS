use crate::net::action::model::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::delete_char::store::DeleteCharStore;
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
    ) -> Result<HandlerResult, NetworkError> {
        let read = DeleteCharRead::new().read_delete_char_packet(packet)?;
        let store = DeleteCharStore::new()
            .store_delete_char(state, session, &read)
            .await?;
        let result = self.build_delete_char_result(state, &store)?;
        Ok(result)
    }

    fn build_delete_char_result(
        &self,
        state: &SharedState,
        store: &DeleteCharStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet = Packet::new_empty()
            .build_delete_char_handler_packet(&store.char.id, &store.status)?
            .finish();
        result.add_action(Action::Local {
            packet: packet.clone(),
        });
        Ok(result)
    }
}
