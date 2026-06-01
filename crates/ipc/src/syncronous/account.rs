use base::account::{FailedCode, PendingCode, StatusCode, SuccessCode};
use bcrypt::verify;
use db::account::model::AccountModel;

use crate::syncronous::error::SyncDomainError;

pub fn check_pic(acc_pic: Option<String>, pic: String) -> bool {
    acc_pic == Some(pic)
}

pub fn authenticate(acc_pw: String, pw: String) -> Result<bool, SyncDomainError> {
    verify(&pw, &acc_pw).map_err(SyncDomainError::BcryptError)
}

pub fn get_status_code_by_account(acc_model: &AccountModel) -> StatusCode {
    if acc_model.banned {
        return StatusCode::Failed(FailedCode::Banned);
    }
    if !acc_model.accepted_tos {
        return StatusCode::Pending(PendingCode::PendingTOS);
    }
    StatusCode::Success(SuccessCode::Success)
}
