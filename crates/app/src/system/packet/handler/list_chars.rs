/* list_chars/store.rs
 * The purpose of this module is to resolve relevant variables for character listing.
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

use crate::component::channel::InChannel;
use crate::component::character::MapleCharacter;
use crate::component::item::MapleItem;
use crate::component::world::InWorld;
use crate::message::packet::list_chars::ListCharsSuccessResponseMessage;
use crate::message::packet::list_chars::ReadListCharsRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::list_chars;
use crate::system::packet::handler::codec::spawn_char;
use crate::system::system_params::InParams;
use crate::system::system_params::LocationParams;
use crate::system::system_params::SessionParams;
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Res};
use config::settings;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;

pub enum PicStatus {
    Disabled = 2,
    AlreadyRegistered = 1,
    NeedsToRegister = 0,
}

pub fn handle_load_char_slots(
    mut commands: Commands,
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<ReadListCharsRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_acc) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((_, acc, _)) = session_params.accounts.get(in_acc.0) else {
            continue;
        };
        let Some((world_entity, _)) = loc_params.worlds.iter().find(|(_, w)| w.id == msg.world_id)
        else {
            continue;
        };
        commands.entity(client_entity).insert(InWorld(world_entity));
        let Some((channel_entity, _, _)) = loc_params
            .channels
            .iter()
            .find(|(_, c, parent)| c.id == msg.channel_id && parent.0 == world_entity)
        else {
            continue;
        };
        commands
            .entity(client_entity)
            .insert(InChannel(channel_entity));
        command_tx
            .0
            .lock()
            .unwrap()
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::ListCharsRequest {
                    client_id: msg.client_id,
                    acc_id: acc.id,
                    channel_id: msg.channel_id,
                    world_id: msg.world_id,
                },
            ))
            .unwrap();
    }
}

pub fn handle_list_chars(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<ListCharsSuccessResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_acc) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((_, acc, _)) = session_params.accounts.get(in_acc.0) else {
            continue;
        };
        let mut chars_map: HashMap<i32, (Entity, MapleCharacter)> = HashMap::new();
        for char_model in msg.char_models.clone() {
            let char: MapleCharacter = MapleCharacter::from(char_model);
            let char_entity = commands.spawn((char.clone(), ChildOf(in_acc.0))).id();
            chars_map.insert(char.id, (char_entity, char.clone()));
        }
        let equips_map: HashMap<i32, Vec<MapleItem>> = spawn_char::spawn_char_with_equips(
            &mut commands,
            chars_map.clone(),
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

        let mut pic_status: i16 = PicStatus::Disabled as i16;
        let Ok(use_pic) = settings::get_pic_required() else {
            continue;
        };
        if acc.pic.clone().is_some() {
            if use_pic {
                pic_status = PicStatus::AlreadyRegistered as i16;
            }
        } else {
            pic_status = PicStatus::NeedsToRegister as i16;
        };

        let Ok(mut list_chars_packet) = list_chars::build_list_chars_packet(
            chars_map,
            equips_map,
            msg.channel_id,
            msg.slots,
            pic_status,
        ) else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::HandlerAction {
                packet: list_chars_packet.finish(),
                scope: ActionScope::Local,
            }],
        });
    }
}
