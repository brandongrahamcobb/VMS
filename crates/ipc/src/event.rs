/* ipc/src/event.rs
 * The purpose of this module is to provide an enum for TCP events.
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

use std::collections::HashMap;

use base::account::{FailedCode, SuccessCode};
use base::map::BaseMap;
use base::mob::BaseMob;
use base::portal::BasePortal;
use base::skill::BaseSkill;
use db::account::model::AccountModel;
use db::character::model::CharacterModel;
use db::item::model::ItemModel;
use db::keybinding::model::KeybindingModel;
use db::skill::model::SkillModel;
use net::packet::model::Packet;

pub enum AsyncEvent {
    // General
    PacketReceived {
        client_id: i32,
        packet: Packet,
    },
    ClientTransitioning {
        client_id: i32,
        channel_id: u8,
        world_id: i16,
    },
    ClientConnected {
        client_id: i32,
    },
    ClientDisconnected {
        client_id: i32,
    },

    // Login
    LoginSuccess {
        client_id: i32,
        acc_id: i32,
        acc_model: AccountModel,
        code: SuccessCode,
    },
    LoginFailed {
        client_id: i32,
        code: FailedCode,
    },
    CharCreationSuccess {
        client_id: i32,
        char_model: CharacterModel,
        equipped_item_model_map: HashMap<i32, Vec<ItemModel>>,
        equip_item_model_map: HashMap<i32, Vec<ItemModel>>,
        use_item_model_map: HashMap<i32, Vec<ItemModel>>,
        etc_item_model_map: HashMap<i32, Vec<ItemModel>>,
        setup_item_model_map: HashMap<i32, Vec<ItemModel>>,
        cash_item_model_map: HashMap<i32, Vec<ItemModel>>,
        keybinding_model_map: HashMap<i32, Vec<KeybindingModel>>,
        skill_model_map: HashMap<i32, Vec<SkillModel>>,
        equip_tab_inv_capacity_map: HashMap<i32, i16>,
        use_tab_inv_capacity_map: HashMap<i32, i16>,
        etc_tab_inv_capacity_map: HashMap<i32, i16>,
        setup_tab_inv_capacity_map: HashMap<i32, i16>,
        cash_tab_inv_capacity_map: HashMap<i32, i16>,
    },
    ListCharsSuccess {
        client_id: i32,
        channel_id: u8,
        char_models: Vec<CharacterModel>,
        equipped_item_model_map: HashMap<i32, Vec<ItemModel>>,
        equip_item_model_map: HashMap<i32, Vec<ItemModel>>,
        use_item_model_map: HashMap<i32, Vec<ItemModel>>,
        etc_item_model_map: HashMap<i32, Vec<ItemModel>>,
        setup_item_model_map: HashMap<i32, Vec<ItemModel>>,
        cash_item_model_map: HashMap<i32, Vec<ItemModel>>,
        keybinding_model_map: HashMap<i32, Vec<KeybindingModel>>,
        skill_model_map: HashMap<i32, Vec<SkillModel>>,
        equip_tab_inv_capacity_map: HashMap<i32, i16>,
        use_tab_inv_capacity_map: HashMap<i32, i16>,
        etc_tab_inv_capacity_map: HashMap<i32, i16>,
        setup_tab_inv_capacity_map: HashMap<i32, i16>,
        cash_tab_inv_capacity_map: HashMap<i32, i16>,
        slots: i16,
        world_id: i16,
    },
    ListCharsFailed {
        client_id: i32,
    },
    CheckCharName {
        client_id: i32,
        exists: bool,
        ign: String,
    },
    SelectCharWithPic {
        client_id: i32,
        char_id: i32,
        status: bool,
    },
    JoinSuccess {
        client_id: i32,
        char_id: i32,
        keybinding_models: Vec<KeybindingModel>,
        skill_models: Vec<SkillModel>,
        equipped_item_models: Vec<ItemModel>,
        equip_tab_item_models: Vec<ItemModel>,
        use_tab_item_models: Vec<ItemModel>,
        etc_tab_item_models: Vec<ItemModel>,
        setup_tab_item_models: Vec<ItemModel>,
        cash_tab_item_models: Vec<ItemModel>,
        equip_tab_capacity: i16,
        use_tab_capacity: i16,
        etc_tab_capacity: i16,
        setup_tab_capacity: i16,
        cash_tab_capacity: i16,
    },
    PickupSuccess {
        client_id: i32,
        item_id: i32,
        ipos: i16,
        pet_pickup: bool,
    },
    CloseAttackSuccess {
        client_id: i32,
        count: i16,
        skill_model: SkillModel,
        base_skill: BaseSkill,
        display: i16,
        toleft: i16,
        stance: i16,
        speed: i16,
        mob_damages: HashMap<u32, Vec<i32>>,
    },
    ChangeMapSuccess {
        client_id: i32,
        base_map: BaseMap,
        base_portals: Vec<BasePortal>,
        base_mobs: Vec<BaseMob>,
    },
}
