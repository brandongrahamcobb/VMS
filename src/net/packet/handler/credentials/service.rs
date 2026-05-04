use crate::config::settings;
use crate::models::account;
use crate::models::account::model::Account;
use crate::net::error::NetworkError;
use crate::runtime::state::SharedState;
use bcrypt::{DEFAULT_COST, hash, verify};

pub enum StatusCode {
    Success = 0,
    Banned = 2,
    InvalidCredentials = 4,
    UnknownCredentials = 5,
    Playing = 7,
    PendingTOS = 23,
}

pub fn authenticate(acc: &Account, pw: &str) -> Result<bool, NetworkError> {
    let hash = hash(&acc.password, DEFAULT_COST)?;
    Ok(verify(pw, &hash)?)
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

async fn check_if_playing(state: SharedState, acc: &Account) -> Result<bool, NetworkError> {
    let session_id: Option<i32> =
        account::query::get_session_id_by_acc_id(state.clone(), &acc.id).await?;
    let playing = match session_id {
        Some(id) => {
            let state = state.lock().await;
            match state.sessions.get(id) {
                Some(session) => session.playing,
                None => false,
            }
        }
        None => false,
    };
    Ok(playing)
}

pub async fn get_status_code(
    state: SharedState,
    acc: &Account,
) -> Result<StatusCode, NetworkError> {
    if check_if_banned(&acc)? {
        return Ok(StatusCode::Banned);
    }
    if check_if_pending_tos(&acc)? {
        return Ok(StatusCode::PendingTOS);
    }
    let mode = settings::get_release_mode()?;
    if check_if_playing(state.clone(), &acc).await? && mode {
        return Ok(StatusCode::Playing);
    }
    return Ok(StatusCode::Success);
}
