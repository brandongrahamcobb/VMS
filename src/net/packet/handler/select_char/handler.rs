use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::select_char::reader::SelectCharReader;
use crate::net::packet::handler::select_char::store::SelectCharStore;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct SelectCharHandler;

impl SelectCharHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: SelectCharReader = SelectCharReader::new().read_select_char_packet(packet)?;
        let store: SelectCharStore SelectCharStore::new().store_select_char(state, session, &reader)?;
        let result: HandlerResult self.build_select_char_result(&store)?;
        Ok(result)
    }

    fn build_select_char_result(
        &self,
        store: &SelectCharStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_select_char_handler_packet(&store.char.id, &store.ip.addr, &store.ip.port)?
            .finish();
        result.add_action(Action::SetChar {
            char: store.char.clone(),
        })?;
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
