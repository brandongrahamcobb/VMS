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

use crate::models::account;
use crate::models::account::model::AccountModel;
use crate::models::character::wrapper::Character;
use crate::models::error::ModelError;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct Account {
    pub model: AccountModel,
    pub chars: Vec<Character>,
}

impl Account {
    pub async fn accept_tos(&self, state: &SharedState) -> Result<Self, ModelError> {
        account::query::setters::accept_tos_by_account_id(state, self.model.get_id()?).await?;
        Ok(self.clone())
    }

    pub async fn set_pic(&self, state: &SharedState, pic: String) -> Result<Self, ModelError> {
        account::query::setters::set_pic_by_account_id(state, self.model.get_id()?, pic.clone())
            .await?;
        Ok(self.clone())
    }
}
