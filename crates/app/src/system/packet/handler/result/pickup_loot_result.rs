/* app/src/system/handler/result/pickup_item_result.rs
 * The purpose of this module is to write the pickup item packet result.
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

use crate::component::character::MapleCharacter;
use crate::message::result::HandlerResult;
use crate::system::packet::build::pickup_loot;

pub fn write_result(
    client_id: i32,
    char: &MapleCharacter,
    item_id: i32,
    pet_pickup: bool,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mode: u8 = 2;
    let mut actions: Vec<Action> = Vec::new();
    let Ok(mut pickup_loot_packet) =
        pickup_loot::build_pickup_loot_packet(char.id, item_id, pet_pickup, mode)
    else {
        return;
    };
    actions.push(Action::HandlerAction {
        packet: pickup_loot_packet.finish(),
        scope: ActionScope::Local,
    });
    results.write(HandlerResult { client_id, actions });
}
