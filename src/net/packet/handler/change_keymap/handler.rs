use crate::models::character::keybinding;
use crate::models::character::keybinding::model::NewKeybinding;
use crate::net::action::model::Action;
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
        let read = ChangeKeymapRead::new().read_change_keymap_packet(packet)?;
        let store = ChangeKeymapStore::new()
            .close_change_keymap(state, session, &read)
            .await?;
        let result = self.build_change_keymap_result(state, session, &store)?;
        Ok(result)
    }

    fn build_change_keymap_actions(
        &self,
        _state: &SharedState,
        _session: &Session,
        _store: &ChangeKeymapStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        result.add_action(Action::Simple)?;
        Ok(result)
    }
}
