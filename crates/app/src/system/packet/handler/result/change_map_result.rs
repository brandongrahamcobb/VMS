/* app/src/system/handler/result/change_map_result.rs
 * The purpose of this module is to write the change map packet result.
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

use action::model::Action;
use action::scope::{ActionScope, MapScope};
use bevy::ecs::message::MessageWriter;

use crate::component::channel::MapleChannel;
use crate::component::character::MapleCharacter;
use crate::component::portal::MaplePortal;
use crate::message::result::HandlerResult;
use crate::system::packet::build::{change_map, codec};

pub fn write_result(
    client_id: i32,
    chars: &Vec<MapleCharacter>,
    channel: &MapleChannel,
    portal: &MaplePortal,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    for char in chars {
        let Ok(mut despawn_packet) = codec::player::spawn::build_despawn_player_packet(char.id)
        else {
            continue;
        };
        let Ok(mut set_field_packet) = change_map::build_set_field_change_map_packet(
            channel.id,
            portal.base.target_map_wz,
            portal.base.wz,
        ) else {
            continue;
        };
        actions.push(Action::HandlerAction {
            packet: despawn_packet.finish(),
            scope: ActionScope::Map(MapScope::SameChannelSameWorld),
        });
        actions.push(Action::HandlerAction {
            packet: set_field_packet.finish(),
            scope: ActionScope::Local,
        });
    }
    results.write(HandlerResult { client_id, actions });
}
