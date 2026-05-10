use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::chat_text::reader::ChatTextReader;
use crate::net::packet::handler::chat_text::store::ChatTextStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{MapScope, Scope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ChatTextHandler;

impl ChatTextHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: ChatTextReader = ChatTextReader::read_chat_text_packet(packet)?;
        let store: ChatTextStore =
            ChatTextStore::store_chat_text(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_chat_text_result(store.clone())?;
        Ok(result)
    }

    fn build_chat_text_result(&self, store: ChatTextStore) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_chat_text_handler_packet(
                store.acc.clone(),
                store.char.clone(),
                store.msg.clone(),
                store.show,
            )?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        })?;
        Ok(result)
    }
}
