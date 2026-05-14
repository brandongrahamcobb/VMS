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
use crate::models::map;
use crate::models::map::wrapper::Map;
use crate::net::error::NetworkError;
use crate::net::packet::handler::enter_cash_shop::reader::EnterCashShopReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct EnterCashShopStore {
    pub acc: Account,
    pub char: Character,
    pub map: Map,
}

impl EnterCashShopStore {
    pub async fn store_enter_cash_shop(
        state: &SharedState,
        session: Session,
        reader: EnterCashShopReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(state);
        std::hint::black_box(reader);
        let acc = session.get_acc()?;
        let char = session.get_active_char(state).await?;
        let map = map::service::get_map_by_id(CASH_SHOP_MAP_ID)?;
        Ok(Self { acc, char, map })
    }
}
