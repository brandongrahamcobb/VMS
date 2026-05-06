use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::tos::reader::TosReader;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct TosHandler;

impl TosHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let read: TosReader = TosReader::new().read_tos_packet(packet)?;
        let store: TosStore = TosStore::new().store_tos(state, session, &reader).await?;
        let result: HandlerResult = self.build_tos_result(&store)?;
        Ok(result)
    }

    fn build_tos_result(&self, store: &TosStore) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        if store.accepted {
            let packet: Packet = Packet::new_empty()
                .build_credentials_handler_successful_login_packet(&store.acc)?
                .finish();
            result.add_action(Action::Send {
                packet: packet.clone(),
                scope: Scope::Local,
            })?;
        }
        Ok(result)
    }
}
