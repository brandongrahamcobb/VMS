use crate::net::error::NetworkError;
use crate::net::packet::handler::login_start::reader::LoginStartReader;
use crate::net::packet::handler::login_start::store::LoginStartStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct LoginStartHandler;

impl LoginStartHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: LoginStartReader = LoginStartReader::read_login_start_packet(packet)?;
        let store: LoginStartStore =
            LoginStartStore::store_login_start(state, session.clone(), reader.clone()).await?;
        let result = self.build_login_start_result(store.clone())?;
        Ok(result)
    }

    fn build_login_start_result(
        &self,
        store: LoginStartStore,
    ) -> Result<HandlerResult, NetworkError> {
        // not implemented
        std::hint::black_box(store);
        let result: HandlerResult = HandlerResult::new();
        Ok(result)
    }
}
