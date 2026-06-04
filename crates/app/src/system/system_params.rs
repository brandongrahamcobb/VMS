use crate::component::account::{InAccount, MapleAccount};
use crate::component::channel::{InChannel, MapleChannel};
use crate::component::character::{InChar, MapleCharacter};
use crate::component::exp::MapleExp;
use crate::component::hp::MapleHealth;
use crate::component::inventory::{MapleEquippedTab, MapleInventory};
use crate::component::keybinding::MapleKeybinding;
use crate::component::map::{InMap, MapleMap};
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
    pub inventories: Query<'w, 's, (Entity, &'static MapleInventory)>,
    pub equipped_tabs: Query<'w, 's, (Entity, &'static MapleEquippedTab)>,
    pub filled_slots: Query<'w, 's, (Entity, &'static MapleFilledItemSlot)>,
    pub empty_slots: Query<'w, 's, (Entity, &'static MapleEmptyItemSlot)>,
}

#[derive(SystemParam)]
pub struct LocationParams<'w, 's> {
    pub worlds: Query<'w, 's, (Entity, &'static MapleWorld)>,
    pub channels: Query<'w, 's, (Entity, &'static MapleChannel, &'static ChildOf)>,
    pub maps: Query<'w, 's, (Entity, &'static MapleMap, &'static ChildOf)>,
}

#[derive(SystemParam)]
pub struct InParams<'w, 's> {
    pub in_worlds: Query<'w, 's, (Entity, &'static InWorld)>,
    pub in_channels: Query<'w, 's, (Entity, &'static InChannel)>,
    pub in_maps: Query<'w, 's, (Entity, &'static InMap)>,
    pub in_sessions: Query<'w, 's, (Entity, &'static InSession)>,
    pub in_accounts: Query<'w, 's, (Entity, &'static InAccount)>,
    pub in_chars: Query<'w, 's, (Entity, &'static InChar)>,
}

#[derive(SystemParam)]
pub struct SessionParams<'w, 's> {
    pub sessions: Query<'w, 's, (Entity, &'static mut MapleSession)>,
    pub accounts: Query<'w, 's, (Entity, &'static mut MapleAccount, &'static ChildOf)>,
    pub chars: Query<'w, 's, (Entity, &'static mut MapleCharacter, &'static ChildOf)>,
    pub keybindings: Query<'w, 's, (Entity, &'static mut MapleKeybinding, &'static ChildOf)>,
    pub skills: Query<'w, 's, (Entity, &'static mut MapleSkill, &'static ChildOf)>,
}

#[derive(SystemParam)]
pub struct StatParams<'w, 's> {
    pub exps: Query<'w, 's, (Entity, &'static mut MapleExp, &'static ChildOf)>,
    pub healths: Query<'w, 's, (Entity, &'static mut MapleHealth, &'static ChildOf)>,
}

#[derive(SystemParam)]
pub struct PositionParams<'w, 's> {
    pub curr_positions:
        Query<'w, 's, (Entity, &'static mut MapleCurrentPosition, &'static ChildOf)>,
    pub last_positions: Query<'w, 's, (Entity, &'static mut MapleLastPosition, &'static ChildOf)>,
}
