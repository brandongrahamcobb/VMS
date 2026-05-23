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

use crate::account::model::AccountModel;
use crate::account::{self, error::AccountEntityError};
use crate::character::wrapper::Character;
use bcrypt::{DEFAULT_COST, hash, verify};
use db::account;
use db::error::DatabaseError;
use diesel::PgConnection;

impl Account {
    pub async fn accept_tos(&self, conn: &PgConnection) -> Result<(), AccountEntityError> {
        account::setters::accept_tos_by_account_id(conn, self.model.get_id()?)
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
        Ok(())
    }

    pub async fn set_pic(&self, pool: &DbPool, pic: String) -> Result<(), AccountEntityError> {
        account::setters::set_pic_by_account_id(conn, self.model.get_id()?, pic.clone())
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
        Ok(())
    }

    pub fn authenticate(&self, pw: String) -> Result<bool, AccountEntityError> {
        let hash =
            hash(self.model.password.clone(), DEFAULT_COST).map_err(AccountEntityError::CryptError)?;
        Ok(verify(&pw, &hash).map_err(AccountEntityError::CryptError)?)
    }

    pub fn check_if_playing(&self, all_acc_ids: Vec<i32>) -> Result<bool, AccountEntityError> {
        let acc_id: i32 = self.model.get_id()?;
        Ok(all_acc_ids.contains(&acc_id))
    }

    pub async fn get_status_code_by_account(
        &self,
        all_acc_ids: Vec<i32>,
    ) -> Result<StatusCode, AccountEntityError> {
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
