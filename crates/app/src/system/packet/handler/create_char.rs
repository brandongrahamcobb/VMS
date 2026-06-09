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

use crate::component::item::MapleItem;
use crate::message::packet::create_char::{
    CreateCharResponseMessage, ReadCreateCharRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::create_char;
use crate::system::packet::handler::codec::spawn_char;
use crate::system::system_params::{InParams, InventoryParams, LocationParams, SessionParams};
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use db::character::model::CharacterModel;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;

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

        let char_model: CharacterModel = ipc::syncronous::char::create_new_char_model(
            acc.id,
            world.id,
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
            .lock()
            .unwrap()
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
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    inv_params: InventoryParams,
    items: Query<(&MapleItem, &ChildOf)>,
    mut messages: MessageReader<CreateCharResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_channel) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let Ok(in_acc) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Some((char_entity, char, _)) = session_params
            .chars
            .iter()
            .find(|(_, c, parent)| c.id == msg.char_id && parent.0 == in_acc.0)
        else {
            continue;
        };
        let chars: Vec<_> = session_params
            .chars
            .iter()
            .filter(|(_, c, parent)| c.id == msg.char_id && parent.0 == in_acc.0)
            .collect();
        spawn_char::spawn_char(
            &mut commands,
            chars,
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
        let Some((_, map, _)) = loc_params
            .maps
            .iter()
            .find(|(_, m, parent)| m.base.wz == char.map_wz && parent.0 == in_channel.0)
        else {
            continue;
        };
        let Some((inv_entity, _, _)) = inv_params
            .inventories
            .iter()
            .find(|(_, _, parent)| parent.0 == char_entity)
        else {
            continue;
        };
        let Some((equipped_tab_entity, _, _)) = inv_params
            .equipped_tabs
            .iter()
            .find(|(_, _, parent)| parent.0 == inv_entity)
        else {
            continue;
        };
        let Some((filled_slot_entity, _, _)) = inv_params
            .filled_slots
            .iter()
            .find(|(_, _, parent)| parent.0 == equipped_tab_entity)
        else {
            continue;
        };
        let equips: Vec<_> = items
            .iter()
            .filter(|(_, parent)| parent.0 == filled_slot_entity)
            .collect();

        let Ok(mut create_char_packet) = create_char::build_create_char_packet(&char, equips, &map)
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::HandlerAction {
                packet: create_char_packet.finish(),
                scope: ActionScope::Local,
            }],
        });
    }
}
