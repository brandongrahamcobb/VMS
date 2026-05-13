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
use crate::models::account::wrapper::Account;
use crate::models::character::wrapper::Character;
use crate::models::shroom::channel::wrapper::Channel;
use crate::models::shroom::map::wrapper::Map;
use crate::models::shroom::world::wrapper::World;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::Scope;

pub enum Action {
    Break { packet: Packet, scope: Scope },
    Set(SetAction),
    Send { packet: Packet, scope: Scope },
}

pub enum SetAction {
    SetMap { map: Map, scope: Scope },
    SetChannel { channel: Channel, scope: Scope },
    SetWorld { world: World, scope: Scope },
    SetAccount { acc: Account },
    SetChar { char: Character },
}
