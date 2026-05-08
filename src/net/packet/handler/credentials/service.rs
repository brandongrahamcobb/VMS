use crate::config::settings;
use crate::models::account::model::AccountModel;
use crate::net::error::NetworkError;
use crate::runtime::state::SharedState;
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Clone)]
pub enum StatusCode {
    Failed(FailedCode),
    Success(SuccessCode),
}

#[derive(Clone)]
pub enum SuccessCode {
    Success = 0,
}

#[derive(Clone)]
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

fn check_if_banned(acc_model: AccountModel) -> Result<bool, NetworkError> {
    if acc_model.banned {
        return Ok(true);
    }
    Ok(false)
}

fn check_if_pending_tos(acc_model: AccountModel) -> Result<bool, NetworkError> {
    if !acc_model.accepted_tos {
        return Ok(true);
    }
    Ok(false)
}

async fn check_if_playing(state: &SharedState, acc_id: i32) -> Result<bool, NetworkError> {
    let state = state.lock().await;
    for session in state.sessions.iter() {
        if session.get_acc()?.model.id == acc_id {
            return Ok(session.playing);
        }
    }
    Ok(false)
}

pub async fn get_status_code_by_account_model(
    state: &SharedState,
    acc_model: AccountModel,
) -> Result<StatusCode, NetworkError> {
    if check_if_banned(acc_model.clone())? {
        return Ok(StatusCode::Failed(FailedCode::Banned));
    }
    if check_if_pending_tos(acc_model.clone())? {
        return Ok(StatusCode::Failed(FailedCode::PendingTOS));
    }
    let mode = settings::get_release_mode()?;
    if check_if_playing(state, acc_model.clone().id).await? & mode {
        return Ok(StatusCode::Failed(FailedCode::Playing));
    }
    return Ok(StatusCode::Success(SuccessCode::Success));
}
