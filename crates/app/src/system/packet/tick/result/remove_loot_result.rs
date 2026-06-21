/* app/src/system/handler/result/remove_loot_result.rs
 * The purpose of this module is to write the remove loot packet tick result.
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
use action::scope::TickScope;
use bevy::ecs::message::MessageWriter;

use crate::message::result::TickResult;
use crate::system::packet::build::codec;

pub fn write_result(
    item_id: i32,
    world_id: i16,
    channel_id: u8,
    map_wz: i32,
    results: &mut MessageWriter<TickResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    let Ok(mut remove_loot_packet) = codec::item::builder::build_remove_loot_packet(item_id) else {
        return;
    };
    actions.push(Action::TickAction {
        packet: remove_loot_packet.finish(),
        scope: TickScope::Map {
            world_id,
            channel_id,
            map_wz,
        },
    });
    results.write(TickResult { actions });
}
