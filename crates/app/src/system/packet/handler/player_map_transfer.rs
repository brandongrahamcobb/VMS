/* app/src/system/packet/handler/player_map_transfer.rs
 * The purpose of this module is to process player map transfer in system messages.
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

use crate::component::item::MapleItem;
use crate::component::map::InMap;
use crate::component::mob::MapleMob;
use crate::component::session::Transitioning;
use crate::message::packet::player_map_transferred::{
    PlayerMapTransferResponseMessage, ReadPlayerMapTransferRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::handler::codec::{lazy_map, load_equips};
use crate::system::packet::handler::result::spawn_char_result;
use crate::system::packet::handler::result::{spawn_mob_controller_result, spawn_mob_result};
use crate::system::system_params::{InParams, InventoryParams, LocationParams, SessionParams};
use action::scope::{ActionScope, MapScope};
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;

pub fn handle_player_map_transfer_request(
    mut commands: Commands,
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    transitioning: Query<&Transitioning>,
    mut messages: MessageReader<ReadPlayerMapTransferRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };
        let Ok(in_session) = in_params.in_sessions.get(client_entity) else {
            continue;
        };
        let Ok(transitioning) = transitioning.get(in_session.0) else {
            continue;
        };

        command_tx
            .0
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::UpdateMapRequest {
                    client_id: msg.client_id,
                    char_id: char.id,
                    map_wz: transitioning.map_wz,
                },
            ))
            .unwrap();

        commands.entity(in_session.0).remove::<Transitioning>();
    }
}

pub fn handle_player_map_transfer_response(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    inv_params: InventoryParams,
    mobs: Query<(&mut MapleMob, &ChildOf)>,
    items: Query<(&MapleItem, &ChildOf)>,
    mut messages: MessageReader<PlayerMapTransferResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_session) = in_params.in_sessions.get(client_entity) else {
            continue;
        };
        commands.entity(in_session.0).remove::<Transitioning>();
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };
        let Ok(in_channel) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let map_entity = if let Some((map_entity, _, _)) = loc_params
            .maps
            .iter()
            .find(|(_, m, parent)| parent.0 == in_channel.0 && m.base.wz == msg.base_map.wz)
        {
            commands.entity(client_entity).insert(InMap(map_entity));
            let mobs: Vec<MapleMob> = mobs
                .iter()
                .filter(|(_, parent)| parent.0 == map_entity)
                .map(|(mob, _)| *mob)
                .collect();
            spawn_mob_result::write_result(msg.client_id, &mobs, &mut results);
            map_entity
        } else {
            let (map_entity, mobs, npcs) = lazy_map::lazy_load_map(
                &mut commands,
                in_channel.0,
                msg.base_map.clone(),
                msg.base_portals.clone(),
                msg.base_mobs.clone(),
            );
            commands.entity(client_entity).insert(InMap(map_entity));
            spawn_mob_result::write_result(msg.client_id, &mobs, &mut results);
            spawn_mob_controller_result::write_result(msg.client_id, &mobs, &mut results);
            map_entity
        };
        let equips_map: HashMap<i32, Vec<MapleItem>> = load_equips::load_equips(
            vec![client_entity],
            &in_params,
            &session_params,
            &inv_params,
            items,
        );
        let Some(equips) = equips_map.get(&char.id) else {
            continue;
        };
        spawn_char_result::write_result(
            msg.client_id,
            &char,
            equips,
            ActionScope::Map(MapScope::SameChannelSameWorld),
            &mut results,
        );
        let other_clients: Vec<_> = client_map
            .0
            .iter()
            .filter(|(_, entity)| {
                in_params
                    .in_maps
                    .get(**entity)
                    .map(|im| im.0 == map_entity)
                    .unwrap_or(false)
            })
            .map(|(_, entity)| *entity)
            .collect();
        let equips_map: HashMap<i32, Vec<MapleItem>> = load_equips::load_equips(
            other_clients,
            &in_params,
            &session_params,
            &inv_params,
            items,
        );
        let Some(equips) = equips_map.get(&char.id) else {
            continue;
        };
        spawn_char_result::write_result(
            msg.client_id,
            &char,
            &equips,
            ActionScope::Local,
            &mut results,
        );
    }
}
