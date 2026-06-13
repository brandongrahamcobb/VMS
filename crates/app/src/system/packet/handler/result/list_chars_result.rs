/* app/src/system/handler/result/list_chars_result.rs
 * The purpose of this module is to write the character listing packet result.
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
use action::scope::{ActionScope, MapScope};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;

use crate::component::account::MapleAccount;
use crate::component::character::MapleCharacter;
use crate::component::item::MapleItem;
use crate::message::result::HandlerResult;
use crate::system::packet::build::list_chars;

pub fn write_result(
    client_id: i32,
    accounts: &Vec<MapleAccount>,
    char_map: &HashMap<i32, (Entity, MapleCharacter)>,
    equips_map: &HashMap<i32, Vec<MapleItem>>,
    channel_id: u8,
    slots: i16,
    pic_status: i16,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    for _ in accounts.iter() {
        let Ok(mut list_chars_packet) = list_chars::build_list_chars_packet(
            &char_map,
            &equips_map,
            channel_id,
            slots,
            pic_status,
        ) else {
            continue;
        };
        actions.push(Action::HandlerAction {
            packet: list_chars_packet.finish(),
            scope: ActionScope::Local,
        });
    }

    results.write(HandlerResult { client_id, actions });
}
