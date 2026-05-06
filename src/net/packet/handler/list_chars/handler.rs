use crate::config::settings;
use crate::models::character::model::Character;
use crate::models::{account, character, world};
use crate::net::action::{Action, LoginAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::list_chars;
use crate::net::packet::handler::list_chars::store::ListCharsStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct ListCharsHandler;

impl ListCharsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: ListCharsReader = ListCharsReader::new().read_list_chars_packet(packet)?;
        let store: ListCharsStore =
            ListCharsStore::new().store_list_chars(state, session, &reader)?;
        let result: HandlerResult = self.build_list_chars_result(state, &store)?;
        Ok(result)
    }

    fn build_list_chars_result(
        state: &SharedState,
        store: &ListCharsStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_list_chars_handler_packet(
                &store.channel.id,
                &store.chars,
                &store.char_max,
                &store.pic_status,
            )
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
