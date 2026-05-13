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

pub async fn get_account_by_map_channel_world_ids_except_self(
    state: &SharedState,
    channel_id: i16,
    map_wz: i32,
    world_id: i16,
    excluded_acc_id: i32,
) -> Result<Vec<Account>, ModelError> {
    let account_models: Vec<AccountModel> =
        account::query::getters::get_account_models_by_channel_world_ids_map_wz(
            state, channel_id, map_wz, world_id,
        )
        .await?;
    let mut accounts: Vec<Account> = Vec::<Account>::new();
    for account_model in account_models {
        if account_model.id != Some(excluded_acc_id) {
            accounts.push(account_model.load(state).await?);
        }
    }
    Ok(accounts)
}

pub async fn get_accounts_by_map_world_ids_except_self(
    state: &SharedState,
    map_wz: i32,
    world_id: i16,
    excluded_acc_id: i32,
) -> Result<Vec<Account>, ModelError> {
    let account_models: Vec<AccountModel> =
        account::query::getters::get_account_models_by_map_wz_world_id(state, map_wz, world_id)
            .await?;
    let mut accounts: Vec<Account> = Vec::<Account>::new();
    for account_model in account_models {
        if account_model.id != Some(excluded_acc_id) {
            accounts.push(account_model.load(state).await?);
        }
    }
    Ok(accounts)
}

pub async fn get_accounts_by_map_wz_except_self(
    state: &SharedState,
    map_wz: i32,
    excluded_acc_id: i32,
) -> Result<Vec<Account>, ModelError> {
    let account_models: Vec<AccountModel> =
        account::query::getters::get_account_models_by_map_wz(state, map_wz).await?;
    let mut accounts: Vec<Account> = Vec::<Account>::new();
    for account_model in account_models {
        if account_model.id != Some(excluded_acc_id) {
            accounts.push(account_model.load(state).await?);
        }
    }
    Ok(accounts)
}

pub async fn get_accounts_by_channel_world_ids_except_self(
    state: &SharedState,
    channel_id: i16,
    world_id: i16,
    excluded_acc_id: i32,
) -> Result<Vec<Account>, ModelError> {
    let account_models: Vec<AccountModel> =
        account::query::getters::get_account_models_by_channel_world_ids(
            state, channel_id, world_id,
        )
        .await?;
    let mut accounts: Vec<Account> = Vec::<Account>::new();
    for account_model in account_models {
        if account_model.id != Some(excluded_acc_id) {
            accounts.push(account_model.load(state).await?);
        }
    }
    Ok(accounts)
}

pub async fn get_accounts_by_channel_id_except_self(
    state: &SharedState,
    channel_id: i16,
    excluded_acc_id: i32,
) -> Result<Vec<Account>, ModelError> {
    let account_models: Vec<AccountModel> =
        account::query::getters::get_account_models_by_channel_id(state, channel_id).await?;
    let mut accounts: Vec<Account> = Vec::<Account>::new();
    for account_model in account_models {
        if account_model.id != Some(excluded_acc_id) {
            accounts.push(account_model.load(state).await?);
        }
    }
    Ok(accounts)
}

pub async fn get_accounts_by_world_id_except_self(
    state: &SharedState,
    world_id: i16,
    excluded_acc_id: i32,
) -> Result<Vec<Account>, ModelError> {
    let account_models: Vec<AccountModel> =
        account::query::getters::get_account_models_by_world_id(state, world_id).await?;
    let mut accounts: Vec<Account> = Vec::<Account>::new();
    for account_model in account_models {
        if account_model.id != Some(excluded_acc_id) {
            accounts.push(account_model.load(state).await?);
        }
    }
    Ok(accounts)
}
