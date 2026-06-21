/* app/src/message/packet/pickup_item.rs
 * The purpose of this module is to serve item pickup packet system messages.
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

use base::{inventory::InventoryTab, map::Point};
use bevy::prelude::Message;

#[derive(Message)]
pub struct ReadPickupItemRequestMessage {
    pub client_id: i32,
    pub item_id: i32,
    pub pet_pickup: bool,
    pub pos: Point,
}

#[derive(Message)]
pub struct PickupItemResponseMessage {
    pub client_id: i32,
    pub count: i16,
    pub item_id: i32,
    pub ipos: i16,
    pub itab: InventoryTab,
    pub pet_pickup: bool,
}
