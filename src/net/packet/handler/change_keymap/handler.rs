// use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::change_keymap::reader::ChangeKeymapReader;
use crate::net::packet::handler::change_keymap::store::ChangeKeymapStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ChangeKeymapHandler;

impl ChangeKeymapHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: ChangeKeymapReader = ChangeKeymapReader::read_change_keymap_packet(packet)?;
        let store: ChangeKeymapStore =
            ChangeKeymapStore::store_change_keymap(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_change_keymap_result(store.clone())?;
        Ok(result)
    }

    fn build_change_keymap_result(
        &self,
        store: ChangeKeymapStore,
    ) -> Result<HandlerResult, NetworkError> {
        // no packet neccessary
        std::hint::black_box(store);
        let result: HandlerResult = HandlerResult::new();
        Ok(result)
    }
}
