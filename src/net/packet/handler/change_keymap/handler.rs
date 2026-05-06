use crate::models::character::keybinding;
use crate::models::character::keybinding::model::NewKeybinding;
use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::change_keymap;
use crate::net::packet::handler::change_keymap::store::ChangeKeymapStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use itertools::izip;

pub struct ChangeKeymapHandler;

impl ChangeKeymapHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: ChangeKeymapReader =
            ChangeKeymapReader::new().read_change_keymap_packet(packet)?;
        let store: ChangeKeymapStore = ChangeKeymapStore::new()
            .close_change_keymap(state, session, &reader)
            .await?;
        let result: HandlerResult = self.build_change_keymap_result(&store)?;
        Ok(result)
    }

    fn build_change_keymap_actions(
        &self,
        store: &ChangeKeymapStore,
    ) -> Result<HandlerResult, NetworkError> {
        // not implemented
        debug!("{:?}", store);
        let mut result: HandlerResult = HandlerResult::new();
        Ok(result)
    }
}
