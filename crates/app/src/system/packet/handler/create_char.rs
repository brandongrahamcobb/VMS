/* app/src/system/handler/create_char.rs
 * The purpose of this module is to handle character creation system messages.
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

use crate::component::character::MapleCharacter;
use crate::component::hp::MapleHealth;
use crate::component::inventory::{
    MapleCashTab, MapleEquipTab, MapleEquippedTab, MapleEtcTab, MapleInventory, MapleSetupTab,
    MapleUseTab,
};
use crate::component::item::MapleItem;
use crate::component::keybinding::MapleKeybinding;
use crate::component::mp::MapleMana;
use crate::component::position::MapleCurrentPosition;
use crate::component::skill::MapleSkill;
use crate::component::slot::{MapleEmptyItemSlot, MapleFilledItemSlot};
use crate::message::packet::create_char::{
    CreateCharResponseMessage, ReadCreateCharRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::handler::result::create_char_result;
use crate::system::system_params::{InParams, LocationParams, SessionParams};
use base::inventory::{
    ANDROID_EQUIP_SLOTS, CASH_EQUIP_SLOTS, PET_EQUIP_SLOTS, REGULAR_EQUIP_SLOTS,
};
use base::skill::BaseSkill;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Res};
use db::character::model::CharacterModel;
use db::item::model::ItemModel;
use db::keybinding::model::KeybindingModel;
use db::skill::model::SkillModel;
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;
use std::collections::HashSet;

pub fn handle_create_char_request(
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<ReadCreateCharRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_world) = in_params.in_worlds.get(client_entity) else {
            continue;
        };
        let Ok((_, world)) = loc_params.worlds.get(in_world.0) else {
            continue;
        };
        let Ok(in_acc) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((_, acc, _)) = session_params.accounts.get(in_acc.0) else {
            continue;
        };

        let char_model: CharacterModel = inc::character::create_new_char_model(
            acc.id,
            world.base.id,
            msg.ign.clone(),
            msg.job_wz,
            msg.face_wz,
            msg.hair_wz,
            msg.hair_color_wz,
            msg.skin_wz,
            msg.gender_wz,
        );

        command_tx
            .0
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::CreateCharRequest {
                    client_id: msg.client_id,
                    char_model,
                    top_wz: msg.top_wz,
                    bottom_wz: msg.bottom_wz,
                    shoes_wz: msg.shoes_wz,
                    weapon_wz: msg.weapon_wz,
                },
            ))
            .unwrap();
    }
}

pub fn handle_create_char_response(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    in_params: InParams,
    mut messages: MessageReader<CreateCharResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_acc) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let char: MapleCharacter = MapleCharacter::from(msg.char_model.clone());
        let char_entity = commands.spawn((char.clone(), ChildOf(in_acc.0))).id();
        let mut hp_map: HashMap<i32, MapleHealth> = HashMap::new();
        let hp: MapleHealth = MapleHealth::from(msg.char_model.clone());
        commands.spawn((hp.clone(), ChildOf(char_entity)));
        hp_map.insert(char.id, hp.clone());
        let mut mp_map: HashMap<i32, MapleMana> = HashMap::new();
        let mp: MapleMana = MapleMana::from(msg.char_model.clone());
        commands.spawn((mp.clone(), ChildOf(char_entity)));
        mp_map.insert(char.id, mp.clone());
        let mut chars: HashMap<i32, (Entity, MapleCharacter)> = HashMap::new();
        chars.insert(msg.char_id, (char_entity, char.clone()));
        let equips_map: HashMap<i32, Vec<MapleItem>> = spawn_new_char(
            &mut commands,
            &chars,
            &hp_map,
            &mp_map,
            &msg.keybinding_model_map,
            &msg.skill_model_map,
            &msg.equipped_item_model_map,
            &msg.equip_item_model_map,
            &msg.use_item_model_map,
            &msg.etc_item_model_map,
            &msg.setup_item_model_map,
            &msg.cash_item_model_map,
            &msg.equip_tab_inv_capacity_map,
            &msg.use_tab_inv_capacity_map,
            &msg.etc_tab_inv_capacity_map,
            &msg.setup_tab_inv_capacity_map,
            &msg.cash_tab_inv_capacity_map,
        );
        let Some(equips) = equips_map.get(&msg.char_id) else {
            continue;
        };
        create_char_result::write_result(msg.client_id, &char, &equips, &hp, &mp, &mut results);
    }
}

pub fn spawn_new_char_equips(
    commands: &mut Commands,
    cid: i32,
    item_map: &HashMap<i32, Vec<ItemModel>>,
    tab_entity: Entity,
) -> Vec<MapleItem> {
    let mut filled_slots: Vec<MapleItem> = Vec::new();
    for (char_id, item_models) in item_map {
        if *char_id == cid {
            for item_model in item_models.clone() {
                let Ok(info) = metadata::item::equip::build_equip_item_wz_info_by_wz(item_model.wz)
                else {
                    continue;
                };
                let item: MapleItem = MapleItem::from((info, item_model));
                if let Some(ipos) = item.ipos {
                    let filled_slot = MapleFilledItemSlot { ipos };
                    let filled_slot_entity =
                        commands.spawn((filled_slot, ChildOf(tab_entity))).id();
                    commands.spawn((item.clone(), ChildOf(filled_slot_entity)));
                    filled_slots.push(item);
                }
            }
        }
    }
    filled_slots
}

pub fn spawn_new_char(
    commands: &mut Commands,
    char_map: &HashMap<i32, (Entity, MapleCharacter)>,
    hp_map: &HashMap<i32, MapleHealth>,
    mp_map: &HashMap<i32, MapleMana>,
    keybinding_model_map: &HashMap<i32, Vec<KeybindingModel>>,
    skill_model_map: &HashMap<i32, Vec<SkillModel>>,
    equipped_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    equip_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    use_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    etc_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    setup_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    cash_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    equip_tab_inv_capacity_map: &HashMap<i32, i16>,
    use_tab_inv_capacity_map: &HashMap<i32, i16>,
    etc_tab_inv_capacity_map: &HashMap<i32, i16>,
    setup_tab_inv_capacity_map: &HashMap<i32, i16>,
    cash_tab_inv_capacity_map: &HashMap<i32, i16>,
) -> HashMap<i32, Vec<MapleItem>> {
    let mut equipped_filled_slots_map: HashMap<i32, Vec<MapleItem>> = HashMap::new();
    for (_, (char_entity, char)) in char_map.iter() {
        let Some(hp) = hp_map.get(&char.id) else {
            continue;
        };
        commands.spawn((hp.clone(), ChildOf(*char_entity)));
        let Some(mp) = mp_map.get(&char.id) else {
            continue;
        };
        commands.spawn((mp.clone(), ChildOf(*char_entity)));
        let curr_pos = MapleCurrentPosition {
            x: 0,
            y: 0,
            fh: None,
        };
        commands.spawn((curr_pos, ChildOf(*char_entity)));
        let Some(keybinding_models) = keybinding_model_map.get(&char.id) else {
            continue;
        };
        for keybinding_model in keybinding_models {
            let keybinding: MapleKeybinding = MapleKeybinding::from(keybinding_model.clone());
            commands.spawn((keybinding, ChildOf(*char_entity)));
        }
        let Some(skill_models) = skill_model_map.get(&char.id) else {
            continue;
        };
        for skill_model in skill_models {
            let base_skill: BaseSkill = BaseSkill { wz: skill_model.wz };
            let skill: MapleSkill = MapleSkill::from((base_skill, skill_model.clone()));
            commands.spawn((skill, ChildOf(*char_entity)));
        }
        let inventory: MapleInventory = MapleInventory;
        let inv_entity = commands.spawn((inventory, ChildOf(*char_entity))).id();
        let equipped_tab: MapleEquippedTab = MapleEquippedTab;
        let equipped_tab_entity = commands.spawn((equipped_tab, ChildOf(inv_entity))).id();
        let Some(equip_tab_capacity) = equip_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        let equip_tab: MapleEquipTab = MapleEquipTab {
            capacity: *equip_tab_capacity,
        };
        let equip_tab_entity = commands.spawn((equip_tab, ChildOf(inv_entity))).id();
        let Some(use_tab_capacity) = use_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        let use_tab: MapleUseTab = MapleUseTab {
            capacity: *use_tab_capacity,
        };
        let use_tab_entity = commands.spawn((use_tab, ChildOf(inv_entity))).id();
        let Some(etc_tab_capacity) = etc_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        let etc_tab: MapleEtcTab = MapleEtcTab {
            capacity: *etc_tab_capacity,
        };
        let etc_tab_entity = commands.spawn((etc_tab, ChildOf(inv_entity))).id();
        let Some(setup_tab_capacity) = setup_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        let setup_tab: MapleSetupTab = MapleSetupTab {
            capacity: *setup_tab_capacity,
        };
        let setup_tab_entity = commands.spawn((setup_tab, ChildOf(inv_entity))).id();
        let Some(cash_tab_capacity) = cash_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        let cash_tab: MapleCashTab = MapleCashTab {
            capacity: *cash_tab_capacity,
        };
        let cash_tab_entity = commands.spawn((cash_tab, ChildOf(inv_entity))).id();

        let equipped_items: Vec<MapleItem> = spawn_new_char_equips(
            commands,
            char.id,
            equipped_item_model_map,
            equipped_tab_entity,
        );
        let equipped_filled_pos: HashSet<i16> =
            equipped_items.iter().filter_map(|s| s.ipos).collect();
        let islots = CASH_EQUIP_SLOTS
            .iter()
            .chain(ANDROID_EQUIP_SLOTS.iter())
            .chain(REGULAR_EQUIP_SLOTS.iter())
            .chain(PET_EQUIP_SLOTS.iter())
            .filter(|islot| !equipped_filled_pos.contains(&islot.key));
        for islot in islots {
            commands.spawn((
                MapleEmptyItemSlot { ipos: islot.key },
                ChildOf(equipped_tab_entity),
            ));
        }
        let equip_filled_slots: Vec<MapleItem> =
            spawn_new_char_equips(commands, char.id, equip_item_model_map, equip_tab_entity);
        let equip_filled_pos: HashSet<i16> =
            equip_filled_slots.iter().filter_map(|s| s.ipos).collect();
        let Some(equip_tab_inv_capacity) = equip_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        for ipos in 0..*equip_tab_inv_capacity {
            if !equip_filled_pos.contains(&ipos) {
                commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(equip_tab_entity)));
            }
        }
        let use_filled_slots: Vec<MapleItem> =
            spawn_new_char_equips(commands, char.id, use_item_model_map, use_tab_entity);
        let use_filled_pos: HashSet<i16> = use_filled_slots.iter().filter_map(|s| s.ipos).collect();
        let Some(use_tab_inv_capacity) = use_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        for ipos in 0..*use_tab_inv_capacity {
            if !use_filled_pos.contains(&ipos) {
                commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(use_tab_entity)));
            }
        }
        let etc_filled_slots: Vec<MapleItem> =
            spawn_new_char_equips(commands, char.id, etc_item_model_map, etc_tab_entity);
        let etc_filled_pos: HashSet<i16> = etc_filled_slots.iter().filter_map(|s| s.ipos).collect();
        let Some(etc_tab_inv_capacity) = etc_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        for ipos in 0..*etc_tab_inv_capacity {
            if !etc_filled_pos.contains(&ipos) {
                commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(etc_tab_entity)));
            }
        }
        let setup_filled_slots: Vec<MapleItem> =
            spawn_new_char_equips(commands, char.id, setup_item_model_map, setup_tab_entity);
        let setup_filled_pos: HashSet<i16> =
            setup_filled_slots.iter().filter_map(|s| s.ipos).collect();
        let Some(setup_tab_inv_capacity) = setup_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        for ipos in 0..*setup_tab_inv_capacity {
            if !setup_filled_pos.contains(&ipos) {
                commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(setup_tab_entity)));
            }
        }
        let cash_filled_slots: Vec<MapleItem> =
            spawn_new_char_equips(commands, char.id, cash_item_model_map, cash_tab_entity);
        let cash_filled_pos: HashSet<i16> =
            cash_filled_slots.iter().filter_map(|s| s.ipos).collect();
        let Some(cash_tab_inv_capacity) = cash_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        for ipos in 0..*cash_tab_inv_capacity {
            if !cash_filled_pos.contains(&ipos) {
                commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(cash_tab_entity)));
            }
        }
        equipped_filled_slots_map.insert(char.id, equipped_items);
    }
    equipped_filled_slots_map
}
