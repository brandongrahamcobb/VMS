use crate::inc::helpers;
use crate::models::account;
use crate::net::error::NetworkError;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::credentials;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::packet::Packet;
use crate::prelude::*;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;

pub struct CredentialsHandler;

impl CredentialsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut pkt_reader = Cursor::new(packet.bytes);
        pkt_reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let user = pkt_reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let pw = pkt_reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        pkt_reader
            .read_bytes(6)
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let hwid = helpers::to_hex_string(
            &pkt_reader
                .read_bytes(4)
                .map_err(ReadError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?,
        );
        let mut result: HandlerResult<LoginAction> = HandlerResult::new();
        match account::query::get_account_by_username(state.clone(), &user).await {
            Ok(acc) => {
                let action = if credentials::service::authenticate(&acc, &pw)? {
                    {
                        let state = state.lock().await;
                        state.sessions.update(session.id as u32, |session| {
                            session.acc_id = Some(acc.id);
                            session.authenticated = true;
                            session.hwid = Some(hwid);
                        });
                    }
                    let status = credentials::service::get_status_code(&acc)?;
                    let packet = if matches!(
                        status,
                        credentials::service::StatusCode::Banned
                            | credentials::service::StatusCode::PendingTOS
                            | credentials::service::StatusCode::Playing
                    ) {
                        Packet::new_empty()
                            .build_credentials_handler_failed_login_packet(status as i8)?
                            .finish()
                    } else {
                        Packet::new_empty()
                            .build_credentials_handler_successful_login_packet(&acc)?
                            .finish()
                    };
                    LoginAction::SendPacket { packet }
                } else {
                    let packet: Packet = Packet::new_empty()
                        .build_credentials_handler_failed_login_packet(
                            credentials::service::StatusCode::InvalidCredentials as i8,
                        )?
                        .finish();
                    LoginAction::SendPacket { packet }
                };
                result.add_action(action)?;
                Ok(result)
            }
            Err(e) if e == diesel::result::Error::NotFound => {
                let packet: Packet = Packet::new_empty()
                    .build_credentials_handler_failed_login_packet(
                        credentials::service::StatusCode::InvalidCredentials as i8,
                    )?
                    .finish();
                let action = LoginAction::SendPacket { packet };
                result.add_action(action)?;
                Ok(result)
            }
            Err(_) => Err(NetworkError::UnexpectedError),
        }
    }
}
