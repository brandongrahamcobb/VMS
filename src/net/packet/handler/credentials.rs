use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::inc::helpers;
use crate::models::account;
use crate::models::account::error::AccountError;
use crate::models::account::model::Account;
use crate::models::error::ModelError;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::error::HandlerError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use bcrypt::{DEFAULT_COST, hash, verify};
use std::io::Cursor;
use std::time::UNIX_EPOCH;

pub enum StatusCode {
    Success = 0,
    Banned = 2,
    InvalidCredentials = 5,
    Playing = 7,
    PendingTOS = 23,
}

pub struct CredentialsHandler;

impl CredentialsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut result = HandlerResult::new();
        let (user, pw, hwid) = read_credentials(packet)?;
        match account::query::get_account_by_username(state.clone(), &user).await {
            Ok(acc) => {
                let action = if authenticate(&acc, &pw)? {
                    let status = get_status_code(&acc)?;
                    let packet = if matches!(
                        status,
                        StatusCode::Banned | StatusCode::PendingTOS | StatusCode::Playing
                    ) {
                        build_failed_login_packet(status as i8)?
                    } else {
                        {
                            let state = state.lock().await;
                            state.sessions.update(session.id as u32, |session| {
                                session.acc_id = Some(acc.id);
                                session.authenticated = true;
                                session.hwid = Some(hwid);
                            });
                        }
                        build_successful_login_packet(&acc)?
                    };
                    LoginAction::SendPacket { packet }
                } else {
                    let packet = build_failed_login_packet(StatusCode::InvalidCredentials as i8)?;
                    LoginAction::SendPacket { packet }
                };
                result.add_action(action)?;
                Ok(result)
            }
            Err(e) if e == diesel::result::Error::NotFound => {
                let packet = build_failed_login_packet(StatusCode::InvalidCredentials as i8)?;
                let action = LoginAction::SendPacket { packet };
                result.add_action(action)?;
                Ok(result)
            }
            Err(e) => Err(NetworkError::from(DatabaseError::from(e))),
        }
    }
}

fn authenticate(acc: &Account, pw: &str) -> Result<bool, NetworkError> {
    let hash = hash(&acc.password, DEFAULT_COST)?;
    match verify(pw, &hash) {
        Ok(true) => Ok(true),
        Ok(false) => Err(NetworkError::from(PacketError::from(
            HandlerError::LoginError,
        ))),
        Err(_) => Err(NetworkError::from(PacketError::from(
            HandlerError::LoginError,
        ))),
    }
}

fn check_if_banned(acc: &Account) -> Result<bool, NetworkError> {
    if acc.banned {
        return Ok(true);
    }
    return Ok(false);
}

fn check_if_pending_tos(acc: &Account) -> Result<bool, NetworkError> {
    if !acc.accepted_tos {
        return Ok(true);
    }
    return Ok(false);
}

fn check_if_playing(acc: &Account) -> Result<bool, NetworkError> {
    if acc.playing {
        return Ok(true);
    }
    return Ok(false);
}

fn get_status_code(acc: &Account) -> Result<StatusCode, NetworkError> {
    if check_if_banned(&acc)? {
        return Ok(StatusCode::Banned);
    }
    if check_if_pending_tos(&acc)? {
        return Ok(StatusCode::PendingTOS);
    }
    if check_if_playing(&acc)? {
        return Ok(StatusCode::Playing);
    }
    return Ok(StatusCode::Success);
}

fn read_credentials(packet: Packet) -> Result<(String, String, String), NetworkError> {
    let mut reader = Cursor::new(packet.bytes);
    reader
        .read_short()
        .map_err(ReadError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    let user = reader
        .read_str_with_length()
        .map_err(ReadError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    let pw = reader
        .read_str_with_length()
        .map_err(ReadError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    reader
        .read_bytes(6)
        .map_err(ReadError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    let hwid = helpers::to_hex_string(
        &reader
            .read_bytes(4)
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?,
    );
    Ok((user, pw, hwid))
}

pub fn build_failed_login_packet(status: i8) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let opcode = SendOpcode::AccountStatus as i16;
    packet
        .write_short(opcode)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(status as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}

pub fn build_successful_login_packet(acc: &Account) -> Result<Packet, NetworkError> {
    let pin_required = settings::get_pin_required()?;
    let mut packet = Packet::new_empty();
    let opcode = SendOpcode::AccountStatus as i16;
    let acc_id = acc.id as i32;
    let gender = acc.gender;
    let account_name = &acc.username;
    let created_at: i64 = acc
        .created_at
        .ok_or(AccountError::MissingField)
        .map_err(ModelError::from)
        .map_err(NetworkError::from)?
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        .try_into()?;
    packet
        .write_short(opcode)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(acc_id)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(gender as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_str_with_length(account_name)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_long(created_at)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(pin_required as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
