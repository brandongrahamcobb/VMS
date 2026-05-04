use crate::models::account;
use crate::net::error::NetworkError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::credentials;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::packet::Packet;
use crate::runtime::state::SharedState;

pub struct CredentialsHandler;

impl CredentialsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let read = credentials::read::read_credentials_packet(packet)?;
        let mut result: HandlerResult<LoginAction> = HandlerResult::new();
        match account::query::get_account_by_username(state.clone(), &read.user).await {
            Ok(acc) => {
                if credentials::service::authenticate(&acc, &read.pw)? {
                    let status = credentials::service::get_status_code(state.clone(), &acc).await?;
                    let packet = if matches!(
                        status,
                        credentials::service::StatusCode::Banned
                            | credentials::service::StatusCode::PendingTOS
                            | credentials::service::StatusCode::Playing
                    ) {
                        let status = status as i8;
                        Packet::new_empty()
                            .build_credentials_handler_failed_login_packet(&status)?
                            .finish()
                    } else {
                        Packet::new_empty()
                            .build_credentials_handler_successful_login_packet(&acc)?
                            .finish()
                    };
                    result.add_action(LoginAction::SendPacket { packet });
                    result.add_action(LoginAction::CreateSession {
                        acc,
                        hwid: read.hwid,
                    });
                } else {
                    let status = credentials::service::StatusCode::InvalidCredentials as i8;
                    let packet: Packet = Packet::new_empty()
                        .build_credentials_handler_failed_login_packet(&status)?
                        .finish();
                    result.add_action(LoginAction::SendPacket { packet })
                };
                Ok(result)
            }
            Err(e) if e == diesel::result::Error::NotFound => {
                let status = credentials::service::StatusCode::UnknownCredentials as i8;
                let packet: Packet = Packet::new_empty()
                    .build_credentials_handler_failed_login_packet(&status)?
                    .finish();
                result.add_action(LoginAction::SendPacket { packet });
                Ok(result)
            }
            Err(_) => Err(NetworkError::UnexpectedError),
        }
    }
}
