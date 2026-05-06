use crate::models::character;
use crate::net::action::model::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::check_char_name;
use crate::net::packet::handler::check_char_name::store::CheckCharNameStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct CheckCharNameHandler;

impl CheckCharNameHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let read = CheckCharNameRead::new().read_check_char_name_packet(packet)?;
        let store = CheckCharNameStore::new()
            .store_check_char_name(state, session, &read)
            .await?;
        let result = build_check_char_name_result(state, session, &store)?;
        Ok(result)
    }

    fn build_check_char_name_result(
        &self,
        _state: &SharedState,
        session: &Session,
        store: &CheckCharNameStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_check_char_name_handler_packet(&store.exists, &store.ign)?
            .finish();
        result.add_action(Action::Local {
            session: session.clone(),
            packet: packet.clone(),
        });
        Ok(result)
    }
}
