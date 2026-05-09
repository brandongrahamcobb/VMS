use crate::models::account;
use crate::models::account::model::Account;
use crate::net::error::NetworkError;
use crate::net::packet::handler::credentials;
use crate::net::packet::handler::credentials::reader::CredentialsReader;
use crate::net::packet::handler::credentials::service::{FailedCode, StatusCode};
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct CredentialsStore {
    pub acc: Option<Account>,
    pub status: StatusCode,
}

impl CredentialsStore {
    pub async fn store_credentials(
        state: &SharedState,
        session: Session,
        reader: CredentialsReader,
    ) -> Result<Self, NetworkError> {
        match account::service::get_account_by_username(state, reader.username.clone()).await {
            Ok(acc) => {
                let status = if credentials::service::authenticate(
                    acc.model.password.clone(),
                    reader.pw.clone(),
                )? {
                    credentials::service::get_status_code_by_account(state, &acc).await?
                } else {
                    StatusCode::Failed(FailedCode::InvalidCredentials)
                };
                {
                    let state = state.lock().await;
                    state.sessions.update(session.id, |s| {
                        s.playing = true;
                    });
                }
                Ok(Self {
                    acc: Some(acc),
                    status,
                })
            }
            Err(e) => {
                let status = StatusCode::Failed(FailedCode::UnknownCredentials);
                Ok(Self { acc: None, status })
            }
        }
    }
}
