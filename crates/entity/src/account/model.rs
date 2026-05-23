/* entity/src/account/model.rs
 * The purpose of this module is to provide an account model.
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

use crate::account::error::AccountEntityError;
use crate::schema::accounts;
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
    pub fn get_id(&self) -> Result<i32, AccountEntityError> {
        if let Some(oid) = self.id {
            Ok(oid)
        } else {
            Err(AccountEntityError::NoId)
        }
    }

    pub fn get_pic(&self) -> Result<String, AccountEntityError> {
        if let Some(pic) = self.pic.clone() {
            Ok(pic)
        } else {
            Err(AccountEntityError::NoPic(self.get_id()?))
        }
    }

    pub fn get_created_at(&self) -> Result<SystemTime, AccountEntityError> {
        if let Some(created_at) = self.created_at {
            Ok(created_at)
        } else {
            Err(AccountEntityError::NoCreatedAt(self.get_id()?))
        }
    }
}
