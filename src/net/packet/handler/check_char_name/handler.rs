use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::check_char_name::reader::CheckCharNameReader;
use crate::net::packet::handler::check_char_name::store::CheckCharNameStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
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
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: CheckCharNameReader = CheckCharNameReader::read_check_char_name_packet(packet)?;
        let store: CheckCharNameStore =
            CheckCharNameStore::store_check_char_name(state, session.clone(), reader.clone()).await?;
        let result = self.build_check_char_name_result(store.clone())?;
        Ok(result)
    }

    fn build_check_char_name_result(
        &self,
        store: CheckCharNameStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_check_char_name_handler_packet(store.exists, store.ign.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
