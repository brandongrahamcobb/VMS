/* account/model.rs
 * The purpose of this module is to provide an account model and its wrapper.
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
use crate::db::schema::accounts;
use crate::models::account::error::AccountError;
use crate::models::account::wrapper::Account;
use crate::models::character;
use crate::models::character::wrapper::Character;
use crate::runtime::state::SharedState;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct AccountModel {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub pin: Option<String>,
    pub pic: Option<String>,
    pub last_login_at: Option<SystemTime>,
    pub gender_wz: i16,
    pub accepted_tos: bool,
    pub banned: bool,
    pub admin: bool,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl AccountModel {
    pub async fn load(&self, state: &SharedState) -> Result<Account, AccountError> {
        let acc_id = self.get_id()?;
        let char_models = character::query::getters::get_char_models_by_account_id(state, acc_id)
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
        let mut chars: Vec<Character> = Vec::<Character>::new();
        for char_model in char_models {
            chars.push(char_model.load(state).await?);
        }
        let acc = Account {
            model: self.clone(),
            chars: chars.clone(),
        };
        Ok(acc)
    }

    pub fn get_id(&self) -> Result<i32, AccountError> {
        if let Some(oid) = self.id {
            Ok(oid)
        } else {
            Err(AccountError::NoId)
        }
    }

    pub fn get_pic(&self) -> Result<String, AccountError> {
        if let Some(pic) = self.pic.clone() {
            Ok(pic)
        } else {
            Err(AccountError::NoPic(self.get_id()?))
        }
    }

    pub fn get_created_at(&self) -> Result<SystemTime, AccountError> {
        if let Some(created_at) = self.created_at {
            Ok(created_at)
        } else {
            Err(AccountError::NoCreatedAt(self.get_id()?))
        }
    }
}
