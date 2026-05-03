use crate::net::error::NetworkError;
use crate::net::packet::error::PacketError;
use crate::{models::account::model::Account, net::packet::handler::error::HandlerError};
use bcrypt::{DEFAULT_COST, hash, verify};

pub enum StatusCode {
    Success = 0,
    Banned = 2,
    InvalidCredentials = 5,
    Playing = 7,
    PendingTOS = 23,
}

pub fn authenticate(acc: &Account, pw: &str) -> Result<bool, NetworkError> {
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

pub fn get_status_code(acc: &Account) -> Result<StatusCode, NetworkError> {
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
