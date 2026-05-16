/* enter_cash_shop/store.rs
 * The purpose of this module is to resolve relevant variables for entering the cash shop.
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

use crate::constants::CASH_SHOP_MAP_ID;
use crate::models::account::wrapper::Account;
use crate::models::character::wrapper::Character;
use crate::models::{account, character};
use crate::net::packet::handler::enter_cash_shop::error::EnterCashShopError;
use crate::net::packet::handler::enter_cash_shop::reader::EnterCashShopReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct EnterCashShopStore {
    pub char: Character,
    pub map_wz: i32,
    pub username: String,
}

impl EnterCashShopStore {
    pub async fn store_enter_cash_shop(
        state: &SharedState,
        session: Session,
        reader: EnterCashShopReader,
    ) -> Result<Self, EnterCashShopError> {
        std::hint::black_box(reader);
        let acc_id: i32 = session.get_acc_id()?;
        let acc: Account = account::service::get_account_by_id(state, acc_id).await?;
        let username: String = acc.model.username;
        let char_id = session.get_char_id()?;
        let char: Character = character::service::get_char_by_id(state, char_id).await?;
        Ok(Self {
            char,
            map_wz: CASH_SHOP_MAP_ID,
            username: username.clone(),
        })
    }
}
