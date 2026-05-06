use crate::net::packet::handler::credentials;
use crate::net::packet::handler::credentials::read::CredentialsRead;
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
        read: &CredentialsRead,
    ) -> Result<Self, NetworkError> {
        match account::query::get_account_by_username(state, &read.user).await {
            Ok(acc) => {
                let status = if credentials::service::authenticate(&acc, &read.pw)? {
                    credentials::service::get_status_code(state, &acc).await?;
                } else {
                    StatusCode::InvalidCredentials as i8;
                };
                if status == StatusCode::Success {
                    let state = state.lock().await;
                    state.sessions.update(&session.id, |s| {
                        s.authenticated = true;
                        s.playing = true;
                    });
                }
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
