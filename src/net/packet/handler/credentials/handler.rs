use crate::net::action::{Action, SetAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::credentials::reader::CredentialsReader;
use crate::net::packet::handler::credentials::service::StatusCode;
use crate::net::packet::handler::credentials::store::CredentialsStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct CredentialsHandler;

impl CredentialsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: CredentialsReader = CredentialsReader::read_credentials_packet(packet)?;
        let store: CredentialsStore =
            CredentialsStore::store_credentials(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_credentials_result(store.clone())?;
        Ok(result)
    }

    fn build_credentials_result(
        &self,
        store: CredentialsStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        match store.status {
            StatusCode::Failed(code) => {
                let code = code as i16;
                let packet: Packet = Packet::new_empty()
                    .build_credentials_handler_failed_login_packet(code)?
                    .finish();
                result.add_action(Action::Send {
                    packet: packet.clone(),
                    scope: Scope::Local,
                })?;
            }
            StatusCode::Success(_) => {
                let packet: Packet = Packet::new_empty()
                    .build_credentials_handler_successful_login_packet(store.acc.clone().unwrap())?
                    .finish();
                result.add_action(Action::Set(SetAction::SetAccount {
                    acc: store.acc.clone().unwrap(),
                }))?;
                result.add_action(Action::Send {
                    packet: packet.clone(),
                    scope: Scope::Local,
                })?;
            }
        }
        Ok(result)
    }
}
