use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::list_chars::reader::ListCharsReader;
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
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: ListCharsReader = ListCharsReader::read_list_chars_packet(packet)?;
        let store: ListCharsStore =
            ListCharsStore::store_list_chars(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_list_chars_result(store.clone())?;
        Ok(result)
    }

    fn build_list_chars_result(
        &self,
        store: ListCharsStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_list_chars_handler_packet(
                store.chars,
                store.channel.id,
                store.char_max as i8,
                store.pic_status,
            )?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
