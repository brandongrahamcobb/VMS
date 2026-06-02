/* create_char/store.rs
 * The purpose of this module is to resolve relevant variables for character creation.
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
use crate::component::character::MapleCharacter;
use crate::component::inventory::{
    MapleCashTab, MapleEquipTab, MapleEquippedTab, MapleEtcTab, MapleInventory, MapleSetupTab,
    MapleUseTab,
};
use crate::component::item::MapleItem;
use crate::component::slot::{MapleEmptyItemSlot, MapleFilledItemSlot};
use crate::component::world::{InWorld, MapleWorld};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::create_char;
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use base::inventory::{
    ANDROID_EQUIP_SLOTS, CASH_EQUIP_SLOTS, PET_EQUIP_SLOTS, REGULAR_EQUIP_SLOTS,
};
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use db::character::model::CharacterModel;
use db::keybinding::model::KeybindingModel;
use db::skill::model::SkillModel;
use ipc::asyncronous::db_command::DatabaseCommand;
use std::collections::HashMap;

pub fn handle_create_char_request(
    commands: Commands,
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    worlds: Query<&MapleWorld>,
    in_worlds: Query<(Entity, &InWorld)>,
    accounts: Query<&MapleAccount>,
    in_accounts: Query<(Entity, &InAccount)>,
    mut messages: MessageReader<CreateCharRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_world_entity, _)) = in_worlds.get(client_entity) else {
            continue;
        };
        let Ok(world) = worlds.get(in_world_entity) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_accounts.get(client_entity) else {
            continue;
        };
        let Ok(acc) = accounts.get(in_acc_entity) else {
            continue;
        };

        let char_model: CharacterModel = ipc::syncronous::char::create_new_char_model(
            acc.id,
            world.id,
            msg.ign,
            msg.job_wz,
            msg.face_wz,
            msg.hair_wz,
            msg.hair_color_wz,
            msg.skin_wz,
            msg.gender_wz,
        );

        commands_tx
            .0
            .lock()
            .unwrap()
            .send(DatabaseCommand::CreateChar {
                client_id: msg.client_id,
                char_model,
            });
    }
}

pub fn handle_create_char_response(
    commands: Commands,
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    accounts: Query<(Entity, &MapleAccount)>,
    in_accounts: Query<(Entity, &InAccount)>,
    in_world: Query<&InWorld>,
    mut messages: MessageReader<CreateCharResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let inv_capacity: i16 = 96;

        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((acc_entity, acc)) = accounts.get(in_acc_entity) else {
            continue;
        };

        let Ok(equip_models) = ipc::syncronous::char::create_new_char_equip_models(
            msg.char_model.id,
            msg.top_wz,
            msg.bottom_wz,
            msg.shoes_wz,
            msg.weapon_wz,
        ) else {
            continue;
        };
        let keybinding_models: Vec<KeybindingModel> =
            ipc::syncronous::char::create_new_char_keybinding_models(msg.char_model.id);
        let skill_models: Vec<SkillModel> = ipc::syncronous::char::create_new_char_skill_models(
            msg.char_model.id,
            msg.char_model.job_wz,
        );

        command_tx
            .0
            .lock()
            .unwrap()
            .send(DatabaseCommand::FinishChar {
                client_id: msg.client_id,
                char_model: msg.char_model,
            });

        let char: MapleCharacter = MapleCharacter::from(msg.char_model);
        let char_entity = commands.spawn((char, ChildOf(acc_entity))).id();

        let inventory: MapleInventory = MapleInventory;
        let inv_entity = commands.spawn((inventory, ChildOf(char_entity))).id();

        let equip_tab: MapleEquipTab = MapleEquipTab {
            capacity: inv_capacity,
        };
        let equip_tab_entity = commands.spawn((equip_tab, ChildOf(inv_entity))).id();
        let use_tab: MapleUseTab = MapleUseTab {
            capacity: inv_capacity,
        };
        let use_tab_entity = commands.spawn((use_tab, ChildOf(inv_entity))).id();
        let etc_tab: MapleEtcTab = MapleEtcTab {
            capacity: inv_capacity,
        };
        let etc_tab_entity = commands.spawn((etc_tab, ChildOf(inv_entity))).id();
        let setup_tab: MapleSetupTab = MapleSetupTab {
            capacity: inv_capacity,
        };
        let setup_tab_entity = commands.spawn((setup_tab, ChildOf(inv_entity))).id();
        let cash_tab: MapleCashTab = MapleCashTab {
            capacity: inv_capacity,
        };
        let cash_tab_entity = commands.spawn((cash_tab, ChildOf(inv_entity))).id();
        for ipos in 0..inv_capacity {
            commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(equip_tab_entity)));
            commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(use_tab_entity)));
            commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(etc_tab_entity)));
            commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(setup_tab_entity)));
            commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(cash_tab_entity)));
        }

        let equipped_tab: MapleEquippedTab = MapleEquippedTab;
        let equipped_tab_entity = commands.spawn((equipped_tab, ChildOf(inv_entity))).id();
        let filled_pos: Vec<i16> = Vec::new();
        let filled_slots: HashMap<MapleFilledItemSlot, MapleItem> = HashMap::new();
        for equip_model in equip_models {
            let Ok(info) = metadata::item::equip::build_equip_item_wz_info_by_wz(equip_model.wz)
            else {
                continue;
            };
            let equip: MapleItem = MapleItem::from((equip_model, info));
            if let Some(ipos) = equip.ipos {
                let filled_slot_entity = commands
                    .spawn((MapleFilledItemSlot { ipos }, ChildOf(equipped_tab_entity)))
                    .id();
                commands.spawn((equip, ChildOf(filled_slot_entity)));
                filled_pos.push(ipos);
            }
        }
        let islots = CASH_EQUIP_SLOTS
            .iter()
            .chain(ANDROID_EQUIP_SLOTS.iter())
            .chain(REGULAR_EQUIP_SLOTS.iter())
            .chain(PET_EQUIP_SLOTS.iter())
            .filter(|islot| !filled_pos.contains(&islot.key));

        for islot in islots {
            commands.spawn((
                MapleEmptyItemSlot { ipos: islot.key },
                ChildOf(equipped_tab_entity),
            ));
        }

        for keybinding_model in keybinding_models {
            let keybinding: MapleKeybinding = MapleKeybinding::from(keybinding_model);
            commands.spawn((keybinding, ChildOf(char_entity)));
        }

        for skill_model in skill_models {
            let skill: MapleSkill = MapleSkill::from(skill_model);
            commands.spawn((skill, ChildOf(char_entity)));
        }

        let Ok(mut create_char_packet) = create_char::build_create_char_packet(&char) else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Session(SessionAction::Send {
                packet: create_char_packet.finish(),
                scope: SessionScope::Local,
            })],
        });
    }
}
