/* account/wrapper.rs
 * The purpose of this module is to provide an account wrapper.
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

use crate::db::error::DatabaseError;
use crate::models::account::model::AccountModel;
use crate::models::account::{self, error::AccountError};
use crate::models::character::wrapper::Character;
use crate::runtime::state::SharedState;
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Clone)]
pub struct Account {
    pub model: AccountModel,
    pub chars: Vec<Character>,
}

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

impl Account {
    pub async fn accept_tos(&self, state: &SharedState) -> Result<Self, AccountError> {
        account::query::setters::accept_tos_by_account_id(state, self.model.get_id()?)
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
        Ok(self.clone())
    }

    pub async fn set_pic(&self, state: &SharedState, pic: String) -> Result<Self, AccountError> {
        account::query::setters::set_pic_by_account_id(state, self.model.get_id()?, pic.clone())
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
        Ok(self.clone())
    }

    pub fn authenticate(&self, pw: String) -> Result<bool, AccountError> {
        let hash =
            hash(self.model.password.clone(), DEFAULT_COST).map_err(AccountError::CryptError)?;
        Ok(verify(&pw, &hash).map_err(AccountError::CryptError)?)
    }

    pub fn check_if_playing(&self, all_acc_ids: Vec<i32>) -> Result<bool, AccountError> {
        let acc_id: i32 = self.model.get_id()?;
        Ok(all_acc_ids.contains(&acc_id))
    }

    pub async fn get_status_code_by_account(
        &self,
        all_acc_ids: Vec<i32>,
    ) -> Result<StatusCode, AccountError> {
        if self.model.banned {
            return Ok(StatusCode::Failed(FailedCode::Banned));
        }
        if !self.model.accepted_tos {
            return Ok(StatusCode::Pending(PendingCode::PendingTOS));
        }
        if self.check_if_playing(all_acc_ids)? {
            return Ok(StatusCode::Failed(FailedCode::Playing));
        }
        return Ok(StatusCode::Success(SuccessCode::Success));
    }
}
