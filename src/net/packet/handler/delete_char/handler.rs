use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::delete_char::reader::DeleteCharReader;
use crate::net::packet::handler::delete_char::store::DeleteCharStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct DeleteCharHandler;

impl DeleteCharHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: DeleteCharReader = DeleteCharReader::read_delete_char_packet(packet)?;
        let store: DeleteCharStore =
            DeleteCharStore::store_delete_char(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_delete_char_result(store.clone())?;
        Ok(result)
    }

    fn build_delete_char_result(
        &self,
        store: DeleteCharStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet = Packet::new_empty()
            .build_delete_char_handler_packet(store.char.model.get_id()?, store.status)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
