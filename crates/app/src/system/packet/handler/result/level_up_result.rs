/* app/src/system/handler/result/level_up_result.rs
 * The purpose of this module is to write the kill mob packet result.
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

use crate::component::character::MapleCharacter;
use crate::message::result::HandlerResult;
use crate::system::packet::build::codec;

pub fn write_result(
    client_id: i32,
    char: &MapleCharacter,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    let Ok(mut set_level_packet) = codec::player::stats::build_set_level_packet(char.level) else {
        return;
    };
    actions.push(Action::HandlerAction {
        packet: set_level_packet.finish(),
        scope: ActionScope::Local,
    });
    let Ok(mut set_exp_packet) = codec::player::stats::build_set_exp_packet(0) else {
        return;
    };
    actions.push(Action::HandlerAction {
        packet: set_exp_packet.finish(),
        scope: ActionScope::Local,
    });
    let Ok(mut level_up_packet) = codec::player::stats::build_level_up_effect_packet(char.id)
    else {
        return;
    };
    actions.push(Action::HandlerAction {
        packet: level_up_packet.finish(),
        scope: ActionScope::Map(MapScope::SameChannelSameWorld),
    });
    results.write(HandlerResult {
        client_id: client_id,
        actions,
    });
}
