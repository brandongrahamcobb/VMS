/* app/src/system/handler/result/mob_damage_result.rs
 * The purpose of this module is to write the mob damage packet result.
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

use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::message::MessageWriter;

use crate::message::result::HandlerResult;
use crate::system::packet::build::codec;

pub fn write_result(
    client_id: i32,
    hp_updates: &HashMap<u32, i16>,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    for (mob_id, hp_percent) in hp_updates {
        let Ok(mut mob_damage_hp_packet) =
            codec::mob::builder::build_mob_damage_show_hp_packet(*mob_id, *hp_percent)
        else {
            continue;
        };
        actions.push(Action::HandlerAction {
            packet: mob_damage_hp_packet.finish(),
            scope: ActionScope::Local,
        });
    }
    results.write(HandlerResult { client_id, actions });
}
