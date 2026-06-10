use base::{map::BaseMap, portal::BasePortal};
/* player_map_transfer/message.rs
 * The purpose of this module is to handle player map transfers.
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
use bevy::prelude::Message;

#[derive(Message)]
pub struct ReadPlayerMapTransferRequestMessage {
    pub client_id: i32,
}

#[derive(Message)]
pub struct PlayerMapTransferResponseMessage {
    pub client_id: i32,
    pub base_map: BaseMap,
    pub base_portals: Vec<BasePortal>,
}
