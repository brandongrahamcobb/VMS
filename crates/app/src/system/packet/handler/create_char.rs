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
use crate::component::item::MapleItem;
use crate::component::world::{InWorld, MapleWorld};
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
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use db::character::model::CharacterModel;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;

pub fn handle_create_char_request(
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    worlds: Query<&MapleWorld>,
    in_worlds: Query<(Entity, &InWorld)>,
    accounts: Query<&MapleAccount>,
    in_accounts: Query<(Entity, &InAccount)>,
    mut messages: MessageReader<ReadCreateCharRequestMessage>,
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
    session_params: SessionParams,
    in_params: InParams,
    inv_params: InventoryParams,
    items: Query<(&MapleItem, &ChildOf)>,
    mut messages: MessageReader<CreateCharResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_channel_entity, _)) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let Ok((channel_entity, _, _)) = loc_params.channels.get(in_channel_entity) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((acc_entity, _, _)) = session_params.accounts.get(in_acc_entity) else {
            continue;
        };
        let Some((char_entity, char, _)) = session_params
            .chars
            .iter()
            .find(|(_, c, parent)| c.id == msg.char_id && parent.0 == acc_entity)
        else {
            continue;
        };
        let chars: Vec<_> = session_params
            .chars
            .iter()
            .filter(|(_, c, parent)| c.id == msg.char_id && parent.0 == acc_entity)
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
            .find(|(_, m, parent)| m.base.wz == char.map_wz && parent.0 == channel_entity)
        else {
            continue;
        };

        let Ok((inv_entity, _)) = inv_params.inventories.get(char_entity) else {
            continue;
        };
        let Ok((equipped_tab_entity, _)) = inv_params.equipped_tabs.get(inv_entity) else {
            continue;
        };
        let Ok((filled_slot_entity, _)) = inv_params.filled_slots.get(equipped_tab_entity) else {
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
