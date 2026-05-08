use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::create_char::reader::CreateCharReader;
use crate::net::packet::handler::create_char::store::CreateCharStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct CreateCharHandler;

impl CreateCharHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: CreateCharReader = CreateCharReader::read_create_character_packet(packet)?;
        let store: CreateCharStore =
            CreateCharStore::store_create_char(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_create_char_result(store.clone())?;
        Ok(result)
    }

    fn build_create_char_result(
        &self,
        store: CreateCharStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_create_char_handler_packet(store.char.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
