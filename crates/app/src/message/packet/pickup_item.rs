/* pickup_item/message.rs
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

use base::map::Point;
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
    pub item_id: i32,
    pub pet_pickup: bool,
    pub ipos: i16,
}
