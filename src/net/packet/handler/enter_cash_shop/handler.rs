/* enter_cash_shop/handler.rs
 * The purpose of this module is to handle cash shop entrance.
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

use crate::net::action::{Action, SetAction};
use crate::net::packet::handler::enter_cash_shop::error::EnterCashShopError;
use crate::net::packet::handler::enter_cash_shop::reader::EnterCashShopReader;
use crate::net::packet::handler::enter_cash_shop::store::EnterCashShopStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{MapScope, Scope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct EnterCashShopHandler;

impl EnterCashShopHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, EnterCashShopError> {
        let reader: EnterCashShopReader = EnterCashShopReader::read_enter_cash_shop_packet(packet)?;
        let store: EnterCashShopStore =
            EnterCashShopStore::store_enter_cash_shop(state, session.clone(), reader).await?;
        let result: HandlerResult = self.build_enter_cash_shop_result(store)?;
        Ok(result)
    }

    fn build_enter_cash_shop_result(
        &self,
        store: EnterCashShopStore,
    ) -> Result<HandlerResult, EnterCashShopError> {
        let mut result: HandlerResult = HandlerResult::new();
        result.add_action(Action::Set(SetAction::SetMap {
            map_wz: store.map_wz,
            scope: Scope::Local,
        }));
        let packet: Packet = Packet::new_empty()
            .build_enter_cash_shop_packet(store.username.clone(), &store.char)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        });
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_packet(&store.char)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        });
        Ok(result)
    }
}
