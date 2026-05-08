use crate::models::account;
use crate::models::account::model::AccountModel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::credentials;
use crate::net::packet::handler::credentials::reader::CredentialsReader;
use crate::net::packet::handler::credentials::service::{FailedCode, StatusCode};
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct CredentialsStore {
    pub acc_model: Option<AccountModel>,
    pub status: StatusCode,
}

impl CredentialsStore {
    pub async fn store_credentials(
        state: &SharedState,
        session: Session,
        reader: CredentialsReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(session);
        match account::query::get_account_model_by_username(state, reader.username.clone()).await {
            Ok(acc_model) => {
                let status = if credentials::service::authenticate(
                    acc_model.password.clone(),
                    reader.pw.clone(),
                )? {
                    credentials::service::get_status_code_by_account_model(
                        state,
                        session.clone(),
                        &acc_model,
                    )
                    .await?
                } else {
                    StatusCode::Failed(FailedCode::InvalidCredentials)
                };
                {
                    let state = state.lock().await;
                    state.sessions.update(session.id, |s| {
                        s.authenticated = true;
                        s.playing = true; // need to track this somehow, db seems to much but
                        // session seems hard idk
                        // s.hwid = hwid;
                    });
                }
                Ok(Self {
                    acc_model: Some(acc_model),
                    status,
                })
            }
            Err(_) => {
                let status = StatusCode::Failed(FailedCode::UnknownCredentials);
                Ok(Self {
                    acc_model: None,
                    status,
                })
            }
        }
    }
}
