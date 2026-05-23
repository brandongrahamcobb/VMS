/* pickup_item/handler.rs
 * The purpose of this module is to handle item pickup.
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

use action::scope::BroadcastScope;
use action::model::{Action, BroadcastAction};
use crate::packet::handler::pickup_item::error::PickupItemEntityError;
use crate::packet::handler::pickup_item::reader::PickupItemReader;
use crate::packet::handler::pickup_item::store::PickupItemStore;
use crate::packet::handler::result::HandlerResult;
use packet::model::Packet;
use db::pool::DbPool;
use session::model::Session;

pub struct PickupItemHandler;

impl PickupItemHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        pool: &DbPool,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, PickupItemEntityError> {
        let reader: PickupItemReader = PickupItemReader::read_pickup_item_packet(packet)?;
        let store: PickupItemStore =
            PickupItemStore::store_pickup_item(pool, session, &reader).await?;
        let result: HandlerResult = self.build_pickup_item_result(&store)?;
        Ok(result)
    }

    fn build_pickup_item_result(
        &self,
        store: &PickupItemStore,
    ) -> Result<HandlerResult, PickupItemEntityError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_pickup_item_packet(store.char_id, store.item_id, store.pet_pickup)?
            .finish();
        result.add_action(Action::Broadcast(BroadcastAction::Send {
            packet: packet.clone(),
            scope: BroadcastScope::Map {
                world_id: store.world_id,
                channel_id: store.channel_id,
                map_wz: store.map_wz,
            },
        }));
        Ok(result)
    }
}
