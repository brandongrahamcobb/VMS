/* app/src/system/system_params.rs
 * The purpose of this module is to provide grouped system queries as system parameters.
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

use crate::component::account::{InAccount, MapleAccount};
use crate::component::channel::{InChannel, MapleChannel};
use crate::component::character::{InChar, MapleCharacter};
use crate::component::hp::MapleHealth;
use crate::component::inventory::{InInventory, MapleEquippedTab, MapleInventory};
use crate::component::keybinding::MapleKeybinding;
use crate::component::map::{InMap, MapleMap};
use crate::component::mp::MapleMana;
use crate::component::position::{MapleCurrentPosition, MapleLastPosition};
use crate::component::session::{InSession, MapleSession};
use crate::component::skill::MapleSkill;
use crate::component::slot::{MapleEmptyItemSlot, MapleFilledItemSlot};
use crate::component::world::{InWorld, MapleWorld};
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::system::{Query, SystemParam};

#[derive(SystemParam)]
pub struct InventoryParams<'w, 's> {
    pub inventories: Query<'w, 's, (Entity, &'static MapleInventory, &'static ChildOf)>,
    pub equipped_tabs: Query<'w, 's, (Entity, &'static MapleEquippedTab, &'static ChildOf)>,
    pub filled_slots: Query<'w, 's, (Entity, &'static MapleFilledItemSlot, &'static ChildOf)>,
    pub empty_slots: Query<'w, 's, (Entity, &'static MapleEmptyItemSlot, &'static ChildOf)>,
}

#[derive(SystemParam)]
pub struct LocationParams<'w, 's> {
    pub worlds: Query<'w, 's, (Entity, &'static MapleWorld)>,
    pub channels: Query<'w, 's, (Entity, &'static MapleChannel, &'static ChildOf)>,
    pub maps: Query<'w, 's, (Entity, &'static MapleMap, &'static ChildOf)>,
}

#[derive(SystemParam)]
pub struct InParams<'w, 's> {
    pub in_worlds: Query<'w, 's, &'static InWorld>,
    pub in_channels: Query<'w, 's, &'static InChannel>,
    pub in_maps: Query<'w, 's, &'static InMap>,
    pub in_sessions: Query<'w, 's, &'static InSession>,
    pub in_accounts: Query<'w, 's, &'static InAccount>,
    pub in_chars: Query<'w, 's, &'static InChar>,
    pub in_inventories: Query<'w, 's, &'static InInventory>,
}

#[derive(SystemParam)]
pub struct SessionParams<'w, 's> {
    pub sessions: Query<'w, 's, (Entity, &'static mut MapleSession, &'static ChildOf)>,
    pub accounts: Query<'w, 's, (Entity, &'static mut MapleAccount, &'static ChildOf)>,
    pub chars: Query<'w, 's, (Entity, &'static mut MapleCharacter, &'static ChildOf)>,
    pub keybindings: Query<'w, 's, (Entity, &'static mut MapleKeybinding, &'static ChildOf)>,
    pub skills: Query<'w, 's, (Entity, &'static mut MapleSkill, &'static ChildOf)>,
}

#[derive(SystemParam)]
pub struct StatParams<'w, 's> {
    pub healths: Query<'w, 's, (&'static mut MapleHealth, &'static ChildOf)>,
    pub manas: Query<'w, 's, (&'static mut MapleMana, &'static ChildOf)>,
}

#[derive(SystemParam)]
pub struct PositionParams<'w, 's> {
    pub curr_positions: Query<'w, 's, (&'static mut MapleCurrentPosition, &'static ChildOf)>,
    pub last_positions: Query<'w, 's, (&'static mut MapleLastPosition, &'static ChildOf)>,
}
