use crate::net::packet::handler::credentials;
use crate::net::packet::handler::credentials::reader::CredentialsReader;
use crate::net::packet::handler::credentials::service::StatusCode;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct CredentialsStore {
    pub status: StatusCode,
}

impl CredentialsStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_credentials(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &CredentialsReader,
    ) -> Result<Self, NetworkError> {
        match account::query::get_account_by_username(state, &reader.user).await {
            Ok(acc) => {
                let status = if credentials::service::authenticate(&acc, &reader.pw)? {
                    credentials::service::get_status_code(state, &acc).await?;
                } else {
                    StatusCode::InvalidCredentials as i8;
                };
                Ok(Self {
                    status: status.clone(),
                })
            }
            Err(e) => {
                let status = StatusCode::UnknownCredentials as i8;
                Ok(Self {
                    status: status.clone(),
                })
            }
        }
    }
}
