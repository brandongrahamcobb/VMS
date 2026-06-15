/* app/src/system/packet/handler/player_logged_in.rs
 * The purpose of this module is to process player logged in system messages.
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
use crate::message::packet::player_logged_in::{
    PlayerLoggedInResponseMessage, ReadPlayerLoggedInRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::handler::codec::load_equips;
use crate::system::packet::handler::result::player_logged_in_result;
use crate::system::system_params::{
    InParams, InventoryParams, LocationParams, SessionParams, StatParams,
};
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Query, Res};
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;

pub fn handle_player_logged_in_request(
    command_tx: Res<CustomSender>,
    mut messages: MessageReader<ReadPlayerLoggedInRequestMessage>,
) -> () {
    for msg in messages.read() {
        command_tx
            .0
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::JoinRequest {
                    client_id: msg.client_id,
                    char_id: msg.char_id,
                },
            ))
            .unwrap();
    }
}

pub fn handle_player_logged_in_response(
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    inv_params: InventoryParams,
    stat_params: StatParams,
    items: Query<(&MapleItem, &ChildOf)>,
    mut messages: MessageReader<PlayerLoggedInResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_channel) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let Ok((_, channel, _)) = loc_params.channels.get(in_channel.0) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };
        let mut binds: Vec<_> = session_params
            .keybindings
            .iter()
            .filter(|(_, _, parent)| parent.0 == in_char.0)
            .collect();
        binds.sort_by_key(|(_, k, _)| k.key);
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
        let Some((hp, _)) = stat_params
            .healths
            .iter()
            .find(|(_, parent)| parent.0 == in_char.0)
        else {
            continue;
        };
        let Some((mp, _)) = stat_params
            .manas
            .iter()
            .find(|(_, parent)| parent.0 == in_char.0)
        else {
            continue;
        };

        player_logged_in_result::write_result(
            msg.client_id,
            channel,
            &binds,
            &char,
            equips,
            &hp,
            &mp,
            &mut results,
        );
    }
}
