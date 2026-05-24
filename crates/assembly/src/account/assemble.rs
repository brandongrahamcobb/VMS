/* entity/src/account/assemble.rs
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

use crate::character;
use db;
use db::account;
use db::pool::DbPool;
use entity::account::model::AccountModel;
use entity::account::wrapper::Account;
use entity::character::wrapper::Character;

use crate::account::error::AccountAssemblyError;

pub async fn assemble_acc_by_username(
    pool: &DbPool,
    username: String,
) -> Result<Account, AccountAssemblyError> {
    let acc_model: AccountModel =
        account::getters::get_acc_model_by_username(pool, username.clone()).await?;
    let acc = assemble_acc_by_id(pool, acc_model.get_id()?).await?;
    Ok(acc)
}

pub async fn assemble_acc_by_id(
    pool: &DbPool,
    acc_id: i32,
) -> Result<Account, AccountAssemblyError> {
    let acc_model = db::account::getters::get_acc_model_by_id(pool, acc_id).await?;
    let char_models = db::character::getters::get_char_models_by_acc_id(pool, acc_id).await?;
    let mut chars: Vec<Character> = Vec::<Character>::new();
    for char_model in char_models {
        chars.push(character::assemble::assemble_char_by_id(pool, char_model.get_id()?).await?);
    }
    let acc = Account {
        model: acc_model,
        chars,
    };
    Ok(acc)
}
