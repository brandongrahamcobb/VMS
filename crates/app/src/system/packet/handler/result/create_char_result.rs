/* app/src/system/handler/result/create_char_result.rs
 * The purpose of this module is to write the character creation packet result.
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
use crate::component::hp::MapleHealth;
use crate::component::item::MapleItem;
use crate::component::mp::MapleMana;
use crate::message::result::HandlerResult;
use crate::system::packet::build::create_char;

pub fn write_result(
    client_id: i32,
    char: &MapleCharacter,
    equips: &Vec<MapleItem>,
    hp: &MapleHealth,
    mp: &MapleMana,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    let Ok(mut create_char_packet) =
        create_char::build_create_char_packet(char, equips, hp, mp, char.spawn_map_wz)
    else {
        return;
    };
    actions.push(Action::HandlerAction {
        packet: create_char_packet.finish(),
        scope: ActionScope::Local,
    });
    results.write(HandlerResult {
        client_id: client_id,
        actions,
    });
}
