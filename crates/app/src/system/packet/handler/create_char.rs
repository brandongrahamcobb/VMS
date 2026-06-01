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

use std::collections::HashMap;

use crate::component::account::MapleAccount;
use crate::component::character::MapleCharacter;
use crate::component::inventory::{
    MapleCashTab, MapleEquipTab, MapleEquippedTab, MapleEtcTab, MapleInventory, MapleSetupTab,
    MapleUseTab,
};
use crate::component::item::MapleItem;
use crate::component::slot::{MapleEmptyItemSlot, MapleFilledItemSlot};
use crate::component::world::InWorld;
use crate::message::packet::create_char::CreateCharMessage;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::create_char;
use crate::system::packet::domain;
use base::inventory::{
    ANDROID_EQUIP_SLOTS, CASH_EQUIP_SLOTS, PET_EQUIP_SLOTS, REGULAR_EQUIP_SLOTS,
};
use base::item::BaseItem;
use bevy::ecs::message::MessageReader;
use bevy::ecs::system::{Commands, Query, Res};
use config::settings;
use db::character::model::CharacterModel;
use db::keybinding::model::KeybindingModel;
use db::skill::model::SkillModel;
use ipc::tcp_command::AsyncCommand;

pub async fn handle_create_char_request(
    client_map: Res<ClientMap>,
    commands: Commands,
    mut messages: MessageReader<CreateCharRequestMessage>,
    command_tx: CustomSender<AsyncCommand>,
    in_accounts: Query<&InAccount>,
    in_world: Query<&InWorld>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_acc) = in_accounts.get(client_entity) else {
            continue;
        };
        let Ok(in_world) = in_worlds.get(client_entity) else {
            continue;
        };

        let char_model: CharacterModel = ipc::sync::char::create_new_char_model(
            in_acc.id,
            in_world.id,
            msg.ign,
            msg.job_wz,
            msg.face_wz,
            msg.hair_wz,
            msg.hair_color_wz,
            msg.skin_wz,
            msg.gender_wz,
        );

        commands_tx.0.send(AsyncCommand::CreateChar {
            client_id: msg.client_id,
            char_model,
        });
    }
}

pub async fn handle_create_char_response(
    client_map: Res<ClientMap>,
    commands: Commands,
    mut messages: MessageReader<CreateCharResponseMessage>,
    command_tx: CustomSender<AsyncCommand>,
    accounts: Query<&MapleAccount>,
    in_world: Query<&InWorld>,
) -> () {
    for msg in messages.read() {
        let inv_capactiy: i8 = 96;

        let Ok(acc) = accounts.get(client_entity) else {
            continue;
        };

        let equip_models: Vec<ItemModel> = ipc::sync::char::create_new_char_equip_models(
            msg.char_model.id,
            top_wz,
            bottom_wz,
            shoes_wz,
            weapon_wz,
        );
        let keybinding_models: Vec<KeybindingModel> =
            ipc::sync::char::create_new_char_keybinding_models(msg.char_model.id);
        let skill_models: Vec<SkillModel> =
            ipc::sync::char::create_new_char_skill_models(msg.char_model.id, msg.char_model.job_wz);

        commands_tx.0.send(AsyncCommand::FinishChar {
            client_id: msg.client_id,
            char_model,
        });

        let char: MapleCharacter = MapleCharacter::from(msg.char_model);
        let char_entity = commands.spawn((char, ChildOf(acc.0))).id();

        let inventory: MapleInventory = MapleInventory;
        let inv_entity = commands.spawn((inventory, ChildOf(char_entity))).id();

        let equip_tab: MapleEquipTab = MapleEquipTab {
            capacity: inv_capacity,
        };
        let equip_tab_entity = commands.spawn((equip_tab, ChildOf(inv_entity))).id();
        let use_tab: MapleUseTab = MapleUseTab {
            capacity: inv_capacity,
        };
        let use_tab_entity = commands.spawn((use_tab, ChildOf(inv_entity)));
        let etc_tab: MapleEtcTab = MapleEtcTab {
            capacity: inv_capacity,
        };
        let etc_tab_entity = commands.spawn((etc_tab, ChildOf(inv_entity)));
        let setup_tab: MapleSetupTab = MapleSetupTab {
            capacity: inv_capacity,
        };
        let setup_tab_entity = commands.spawn((setup_tab, ChildOf(inv_entity)));
        let cash_tab: MapleCashTab = MapleCashTab {
            capacity: inv_capacity,
        };
        let cash_tab_entity = commands.spawn((cash_tab, ChildOf(inv_entity)));
        for ipos in 0..inv_capacity {
            commands.spawn((EmptySlot { ipos }, ChildOf(equip_tab_entity)));
            commands.spawn((EmptySlot { ipos }, ChildOf(use_tab_entity)));
            commands.spawn((EmptySlot { ipos }, ChildOf(etc_tab_entity)));
            commands.spawn((EmptySlot { ipos }, ChildOf(setup_tab_entity)));
            commands.spawn((EmptySlot { ipos }, ChildOf(cash_tab_entity)));
        }

        let equipped_tab: MapleEquippedTab = MapleEquippedTab;
        let equipped_tab_entity = commands.spawn((equipped_tab, ChildOf(inv_entity)));
        let filled_pos: Vec<i16> = Vec::new();
        let filled_slots: HashMap<MapleFilledItemSlot, MapleItem> = HashMap::new();
        for equip_models in equip_models {
            let info: BaseItem =
                metadata::item::equip::build_equip_item_wz_info_by_wz(equip_model.wz)
            else {
                continue;
            };
            let equip: MapleItem = MapleItem::from((equip_model, info));
            let filled_slot_entity = commands
                .spawn((
                    MapleFilledItemSlot { ipos: equip.ipos },
                    ChildOf(equipped_tab_entity),
                ))
                .id();
            commands.spawn((equip, ChildOf(filled_slot_entity)));
            filled_pos.push(equip.ipos);
        }
        let islots = [
            CASH_EQUIP_SLOTS,
            ANDROID_EQUIP_SLOTS,
            REGULAR_EQUIP_SLOTS,
            PET_EQUIP_SLOTS,
        ];
        let flat_islots = islots.flatten();

        let empty_slots: Vec<MapleEmptyItemSlot> = Vec::new();
        for (i, islot) in flat_islots.iter().enumerate() {
            if let Some(fill_pos) = filled_pos.iter().find(|f| f == islot.key) {
                flat_islots.remove(i);
            }
        }

        for islot in flat_islots.iter() {
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

        let Ok(create_char_packet) = create_char::build_create_char_packet(&char) else {
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
