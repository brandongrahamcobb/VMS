/* app/src/system/handler/result/player_logged_in_result.rs
 * The purpose of this module is to write the player logged in packet result.
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
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::MessageWriter;

use crate::component::channel::MapleChannel;
use crate::component::character::MapleCharacter;
use crate::component::item::MapleItem;
use crate::component::keybinding::MapleKeybinding;
use crate::message::result::HandlerResult;
use crate::system::packet::build::{codec, player_logged_in};

pub fn write_result(
    client_id: i32,
    char_map: &HashMap<MapleCharacter, Vec<MapleItem>>,
    binds: &Vec<(Entity, &MapleKeybinding, &ChildOf)>,
    channel: &MapleChannel,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    for (char, equips) in char_map.iter() {
        let Ok(mut keymap_packet) = player_logged_in::build_player_logged_in_keymap_packet(&binds)
        else {
            continue;
        };
        actions.push(Action::HandlerAction {
            packet: keymap_packet.finish(),
            scope: ActionScope::Local,
        });
        let Ok(mut set_field_packet) =
            codec::player::set_field::build_set_field_packet(char, equips, channel.id)
        else {
            continue;
        };
        actions.push(Action::HandlerAction {
            packet: set_field_packet.finish(),
            scope: ActionScope::Local,
        });
    }
    results.write(HandlerResult { client_id, actions });
}
