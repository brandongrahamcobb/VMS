/* app/src/system/handler/result/select_char_result.rs
 * The purpose of this module is to write the character selection packet result.
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
use action::scope::ActionScope;
use bevy::ecs::message::MessageWriter;
use config::settings;
use inc::helpers;

use crate::component::channel::MapleChannel;
use crate::message::result::HandlerResult;
use crate::system::packet::build::codec;

pub fn write_result(
    client_id: i32,
    char_ids: &Vec<i32>,
    channel: &MapleChannel,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    for char_id in char_ids.iter() {
        let Ok(addr) = settings::get_routing_address() else {
            continue;
        };
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);

        let Ok(mut select_char_packet) =
            codec::login::builder::build_select_char_packet(*char_id, octets, channel.port)
        else {
            continue;
        };
        actions.push(Action::HandlerAction {
            packet: select_char_packet.finish(),
            scope: ActionScope::Local,
        });
    }
    results.write(HandlerResult { client_id, actions });
}
