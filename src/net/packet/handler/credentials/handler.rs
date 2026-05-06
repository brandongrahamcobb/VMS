use crate::models::account;
use crate::net::action::{Action, LoginAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::credentials;
use crate::net::packet::handler::credentials::reader::CredentialsReader;
use crate::net::packet::handler::credentials::store::CredentialsStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
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
        let reader: CredentialsReader = CredentialsReader::new().read_credentials_packet(packet)?;
        let store: CredentialsStore =
            CredentialsStore::new().store_credentials(state, session, &reader)?;
        let result: HandlerResult = self.build_credentials_result(&store)?;
        Ok(result)
    }

    fn build_credentials_result(
        &self,
        store: &CredentialsStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let status = *store.status as i8;
        match &store.status {
            StatusCode::Failed => {
                let packet: Packet = Packet::new_empty()
                    .build_credentials_handler_failed_login_packet(&status)?
                    .finish();
                result.add_action(Action::Send {
                    packet: packet.clone(),
                    scope: Scope::Local,
                })?;
            }
            StatusCode::Success => {
                let packet: Packet = Packet::new_empty()
                    .build_credentials_handler_successful_login_packet(&store.acc)?
                    .finish();
                result.add_action(Action::Set(SetAction::SetAuthenticated))?;
                result.add_action(Action::Set(SetAction::SetAccount { acc: acc.clone() }))?;
                result.add_action(Action::Set(SetAction::SetHwid { hwid: hwid.clone() }))?;
                result.add_action(Action::Send {
                    packet: packet.clone(),
                    scope: Scope::Local,
                })?;
            }
        }
        Ok(result)
    }
}
