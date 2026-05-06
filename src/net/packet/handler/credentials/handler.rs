use crate::models::account;
use crate::net::action::model::{Action, LoginAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::credentials;
use crate::net::packet::handler::credentials::store::CredentialsStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::state::SharedState;

pub struct CredentialsHandler;

impl CredentialsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let read = CredentialsRead::new().read_credentials_packet(packet)?;
        let store = CredentialsStore::new().store_credentials(state, session, &read)?;
        let result = self.build_credentials_result(state, &store)?;
        Ok(result)
    }

    fn build_credentials_result(
        &self,
        _state: &SharedState,
        store: &CredentialsStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let status = *store.status as i8;
        match &store.status {
            StatusCode::Failed => {
                let packet: Packet = Packet::new_empty()
                    .build_credentials_handler_failed_login_packet(&status)?
                    .finish();
                result.add_action(Action::Local {
                    packet: packet.clone(),
                });
            }
            StatusCode::Success => {
                let packet: Packet = Packet::new_empty()
                    .build_credentials_handler_successful_login_packet(&store.acc)?
                    .finish();
                result.add_action(Action::Local {
                    packet: packet.clone(),
                });
                result.add_action(Action::Login(LoginAction::CreateSession {
                    acc_id: store.acc.id.clone(),
                    hwid: store.hwid.clone(),
                }));
            }
        }
        Ok(result)
    }
}
