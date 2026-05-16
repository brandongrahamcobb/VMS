/* action.rs
 * The purpose of this module is to provide actions to take after handling an incoming packet.
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
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::Scope;

pub enum Action {
    Break { packet: Packet, scope: Scope },
    Retrieve,
    Set(SetAction),
    Send { packet: Packet, scope: Scope },
}

pub enum SetAction {
    SetMap { map_wz: i32, scope: Scope },
    SetChannel { channel_id: u8, scope: Scope },
    SetWorld { world_id: i16, scope: Scope },
    SetAccount { acc_id: i32 },
    SetChar { char_id: i32 },
}
