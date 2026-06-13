/* app/src/system/handler/result/chat_text_result.rs
 * The purpose of this module is to write the chat text packet result.
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
use bevy::ecs::message::MessageWriter;

use crate::component::account::MapleAccount;
use crate::component::character::MapleCharacter;
use crate::message::result::HandlerResult;
use crate::system::packet::build::chat_text;

pub fn write_result(
    client_id: i32,
    acc: &MapleAccount,
    char: &MapleCharacter,
    messages: &HashMap<String, i16>,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    for (msg, show) in messages.iter() {
        let Ok(mut chat_packet) =
            chat_text::build_chat_text_packet(acc.admin, char.id, msg.clone(), *show)
        else {
            continue;
        };
        actions.push(Action::HandlerAction {
            packet: chat_packet.finish(),
            scope: ActionScope::Map(MapScope::SameChannelSameWorld),
        });
    }
    results.write(HandlerResult { client_id, actions });
}
