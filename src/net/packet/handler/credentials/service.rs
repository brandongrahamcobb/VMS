/* credentials/service.rs
 * The purpose of this module is to provide assisting functions and implementations for credential validation.
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

use crate::models::account::wrapper::Account;
use crate::net::error::NetworkError;
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Clone)]
pub enum StatusCode {
    Failed(FailedCode),
    Pending(PendingCode),
    Success(SuccessCode),
}

#[derive(Clone)]
pub enum PendingCode {
    PendingTOS = 23,
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
}

pub fn authenticate(db_pw: String, pw: String) -> Result<bool, NetworkError> {
    let hash = hash(db_pw, DEFAULT_COST)?;
    Ok(verify(&pw, &hash)?)
}

pub async fn get_status_code_by_account(acc: &Account) -> Result<StatusCode, NetworkError> {
    if acc.model.banned {
        return Ok(StatusCode::Failed(FailedCode::Banned));
    }
    if !acc.model.accepted_tos {
        return Ok(StatusCode::Pending(PendingCode::PendingTOS));
    }
    if acc.model.playing {
        return Ok(StatusCode::Failed(FailedCode::Playing));
    }
    return Ok(StatusCode::Success(SuccessCode::Success));
}
