use crate::models::account::model::AccountModel;
use crate::net::error::NetworkError;
use crate::runtime::state::SharedState;
use crate::{config::settings, runtime::session::Session};
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Copy)]
pub enum StatusCode {
    Failed(FailedCode),
    Success(SuccessCode),
}

pub enum SuccessCode {
    Success = 0,
}
pub enum FailedCode {
    Banned = 2,
    InvalidCredentials = 4,
    UnknownCredentials = 5,
    Playing = 7,
    PendingTOS = 23,
}

pub fn authenticate(db_pw: String, pw: String) -> Result<bool, NetworkError> {
    let hash = hash(db_pw, DEFAULT_COST)?;
    Ok(verify(&pw, &hash)?)
}

fn check_if_banned(acc: &AccountModel) -> Result<bool, NetworkError> {
    if acc.banned {
        return Ok(true);
    }
    return Ok(false);
}

fn check_if_pending_tos(acc: &AccountModel) -> Result<bool, NetworkError> {
    if !acc.accepted_tos {
        return Ok(true);
    }
    return Ok(false);
}

async fn check_if_playing(
    state: &SharedState,
    session: Session,
    acc: &AccountModel,
) -> Result<bool, NetworkError> {
    let playing = match session.id {
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

pub async fn get_status_code_by_account_model(
    state: &SharedState,
    session: Session,
    acc: &AccountModel,
) -> Result<StatusCode, NetworkError> {
    if check_if_banned(acc)? {
        return Ok(StatusCode::Failed(FailedCode::Banned));
    }
    if check_if_pending_tos(acc)? {
        return Ok(StatusCode::Failed(FailedCode::PendingTOS));
    }
    let mode = settings::get_release_mode()?;
    if check_if_playing(state, session, acc).await? & mode {
        return Ok(StatusCode::Failed(FailedCode::Playing));
    }
    return Ok(StatusCode::Success(SuccessCode::Success));
}
