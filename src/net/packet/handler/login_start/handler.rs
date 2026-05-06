use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::state::SharedState;

pub struct LoginStartHandler;

impl LoginStartHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: LoginStartReader = LoginStartReader::new().read_login_start_packet(packet)?;
        let store: LoginStartStore =
            LoginStartStore::new().store_login_start(state, session, &reader)?;
        let result = self.build_login_start_result(&store)?;
        Ok(result)
    }

    fn build_login_start_result(
        &self,
        store: &LoginStartStore,
    ) -> Result<HandlerResult, NetworkError> {
        // not implemented
        let result: HandlerResult = HandlerResult::new();
        Ok(result)
    }
}
