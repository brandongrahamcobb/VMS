/* entity/src/account/service.rs
 * The purpose of this module is to provide assisting functions for accounts.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::account::wrapper::{FailedCode, PendingCode, StatusCode, SuccessCode};
use crate::account::{error::AccountEntityError, model::AccountModel};
use bcrypt::{DEFAULT_COST, hash, verify};

pub fn check_pic(acc_pic: Option<String>, pic: String) -> bool {
    if acc_pic == Some(pic) {
        return true;
    } else {
        return false;
    }
}

pub fn authenticate(acc_pw: String, pw: String) -> Result<bool, AccountEntityError> {
    let hash = hash(acc_pw, DEFAULT_COST).map_err(AccountEntityError::CryptError)?;
    Ok(verify(&pw, &hash).map_err(AccountEntityError::CryptError)?)
}

pub fn check_if_playing(all_acc_ids: Vec<i32>, acc_id: i32) -> bool {
    all_acc_ids.contains(&acc_id)
}

pub async fn get_status_code_by_account(
    all_acc_ids: Vec<i32>,
    acc_model: AccountModel,
) -> Result<StatusCode, AccountEntityError> {
    if acc_model.banned {
        return Ok(StatusCode::Failed(FailedCode::Banned));
    }
    if !acc_model.accepted_tos {
        return Ok(StatusCode::Pending(PendingCode::PendingTOS));
    }
    if check_if_playing(all_acc_ids, acc_model.get_id()?) {
        return Ok(StatusCode::Failed(FailedCode::Playing));
    }
    return Ok(StatusCode::Success(SuccessCode::Success));
}
