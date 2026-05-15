/* account/service.rs
 * The purpose of this module is to provide assisting functions and implementations for accounts.
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

use crate::models::account;
use crate::models::account::model::AccountModel;
use crate::models::account::wrapper::Account;
use crate::models::error::ModelError;
use crate::runtime::state::SharedState;

pub async fn get_account_by_username(
    state: &SharedState,
    username: String,
) -> Result<Account, ModelError> {
    let acc_model: AccountModel =
        account::query::getters::get_account_model_by_username(state, username.clone()).await?;
    let acc = acc_model.load(state).await?;
    Ok(acc)
}

pub async fn get_account_by_id(state: &SharedState, acc_id: i32) -> Result<Account, ModelError> {
    let acc_model: AccountModel =
        account::query::getters::get_account_model_by_id(state, acc_id).await?;
    let acc = acc_model.load(state).await?;
    Ok(acc)
}

pub fn check_pic(acc_pic: Option<String>, pic: String) -> Result<bool, ModelError> {
    if acc_pic == Some(pic) {
        return Ok(true);
    } else {
        return Ok(false);
    }
}
