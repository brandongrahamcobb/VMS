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

use crate::packet::handler::enter_cash_shop::constants::CASH_SHOP_MAP_ID;
use crate::packet::handler::enter_cash_shop::error::EnterCashShopError;
use db::pool::DbPool;
use entity::account::wrapper::Account;
use entity::character::wrapper::Character;
use session::model::Session;

pub struct EnterCashShopStore {
    pub char: Character,
    pub map_wz: i32,
    pub username: String,
}

impl EnterCashShopStore {
    pub async fn store_enter_cash_shop(
        pool: &DbPool,
        session: &Session,
    ) -> Result<Self, EnterCashShopError> {
        let acc_id: i32 = session.get_acc_id()?;
        let acc: Account = assembly::account::assemble::assemble_acc_by_id(pool, acc_id).await?;
        let username: String = acc.model.username;
        let char_id = session.get_char_id()?;
        let char: Character =
            assembly::character::assemble::assemble_char_by_id(pool, char_id).await?;
        Ok(Self {
            char,
            map_wz: CASH_SHOP_MAP_ID,
            username: username.clone(),
        })
    }
}
