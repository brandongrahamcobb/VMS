/* pickup_item/store.rs
 * The purpose of this module is to resolve relevant variables for player login.
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

use crate::pickup_item::error::PickupItemEntityError;
use crate::pickup_item::reader::PickupItemReader;
use db::pool::DbPool;
use domain;
use entity::character::wrapper::Character;
use entity::item::wrapper::{Inventory, Item};
use entity::map::model::Point;
use session::model::Session;

pub struct PickupItemStore {
    pub char_id: i32,
    pub item_id: i32,
    pub world_id: i16,
    pub channel_id: u8,
    pub map_wz: i32,
    pub pet_pickup: bool,
    pub pos: Point,
}

impl PickupItemStore {
    pub async fn store_pickup_item(
        pool: &DbPool,
        session: &Session,
        reader: &PickupItemReader,
    ) -> Result<Self, PickupItemEntityError> {
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = session.get_channel_id()?;
        let map_wz: i32 = session.get_map_wz()?;
        let char_id: i32 = session.get_char_id()?;
        let char: Character =
            assembly::character::assemble::assemble_char_by_id(pool, char_id).await?;
        let mut inv: Inventory =
            assembly::item::assemble::assemble_inventory_by_char_id(pool, char_id).await?;
        let item: Item =
            assembly::item::assemble::assemble_item_by_id(pool, reader.item_id).await?;
        domain::item::pick_up(pool, &mut inv, char.model.ign.clone(), char_id, item).await?;
        let pet_pickup: bool = false; //placeholder
        Ok(Self {
            char_id,
            item_id: reader.item_id,
            world_id,
            channel_id,
            map_wz,
            pet_pickup,
            pos: reader.pos.clone(),
        })
    }
}
