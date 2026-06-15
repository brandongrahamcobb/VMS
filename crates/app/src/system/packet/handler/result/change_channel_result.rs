/* app/src/system/handler/result/change_channel_result.rs
 * The purpose of this module is to write the  packet result.
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
use config::settings;
use inc::helpers;

use crate::component::channel::MapleChannel;
use crate::component::character::MapleCharacter;
use crate::message::result::HandlerResult;
use crate::system::packet::build::{cc, codec};

pub fn write_result(
    client_id: i32,
    channel: &MapleChannel,
    char: &MapleCharacter,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    let Ok(addr) = settings::get_routing_address() else {
        return;
    };
    let octets: [u8; 4] = helpers::convert_to_ip_array(addr);
    let Ok(mut despawn_packet) = codec::player::spawn::build_despawn_player_packet(char.id) else {
        return;
    };
    let Ok(mut cc_packet) = cc::build_channel_change_packet(octets, channel.port) else {
        return;
    };
    actions.push(Action::HandlerAction {
        packet: despawn_packet.finish(),
        scope: ActionScope::Map(MapScope::SameChannelSameWorld),
    });
    actions.push(Action::HandlerAction {
        packet: cc_packet.finish(),
        scope: ActionScope::Local,
    });
    results.write(HandlerResult { client_id, actions });
}
