/* move_player/store.rs
 * The purpose of this module is to resolve relevant variables for player movement.
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

use crate::message::packet::player_moved::ReadPlayerMovedRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::player_moved;
use crate::system::system_params::{InParams, PositionParams, SessionParams};
use action::model::Action;
use action::scope::{ActionScope, MapScope};
use base::map::Point;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Res;
use ipc::syncronous;

pub fn handle_player_moved(
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    mut pos_params: PositionParams,
    mut messages: MessageReader<ReadPlayerMovedRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        if !msg.too_short && !msg.empty {
            let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
                continue;
            };
            let Ok((in_char_entity, _)) = in_params.in_chars.get(client_entity) else {
                continue;
            };
            let Ok((char_entity, char, _)) = session_params.chars.get(in_char_entity) else {
                continue;
            };
            let Ok((_, mut curr_pos, _)) = pos_params.curr_positions.get_mut(char_entity) else {
                continue;
            };
            let new_pos: Point = syncronous::map::parse_position(&msg.movement_bytes)
                .unwrap_or(Point { x: 0, y: 0 });
            curr_pos.x = new_pos.x;
            curr_pos.y = new_pos.y;

            let Ok(mut player_moved_packet) =
                player_moved::build_player_move_packet(char.id, msg.movement_bytes.clone())
            else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::HandlerAction {
                    packet: player_moved_packet.finish(),
                    scope: ActionScope::Map(MapScope::SameChannelSameWorld),
                }],
            });
        }
    }
}
