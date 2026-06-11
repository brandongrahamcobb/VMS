/* app/src/component/mob.rs
 * The purpose of this module is to provide a mob component.
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

use crate::component::mob::MapleMob;
use crate::message::result::HandlerResult;
use crate::system::packet::build::codec;

pub fn write_result(
    client_id: i32,
    mobs: Vec<MapleMob>,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let stance: i8 = 0; //placeholder
    let effect: i8 = 0; //placeholder
    let team: i8 = -1; //placeholder
    let mode: u8 = 1; //placeholder
    for mob in mobs.iter() {
        let Ok(mut spawn_mob_controller_packet) =
            codec::mob::builder::build_spawn_mob_controller_packet(mob, mode, stance, effect, team)
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: client_id,
            actions: vec![Action::HandlerAction {
                packet: spawn_mob_controller_packet.finish(),
                scope: ActionScope::Local,
            }],
        });
    }
}
