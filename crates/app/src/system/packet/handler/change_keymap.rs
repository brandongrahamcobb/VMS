/* change_keymap/store.rs
 * The purpose of this module is to resolve relevant variables for changing keymaps.
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

use crate::component::character::{InChar, MapleCharacter};
use crate::message::packet::change_keymap::ReadChangeKeymapRequestMessage;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageReader;
use bevy::ecs::system::{Query, Res};
use db::keybinding::model::KeybindingModel;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;
use itertools::izip;
use std::time::SystemTime;

pub fn handle_change_keymap(
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    chars: Query<&MapleCharacter>,
    in_chars: Query<(Entity, &InChar)>,
    mut messages: MessageReader<ReadChangeKeymapRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_chars.get(client_entity) else {
            continue;
        };
        let Ok(char) = chars.get(in_char_entity) else {
            continue;
        };

        let binds: Vec<KeybindingModel> =
            izip!(msg.keys.clone(), msg.types.clone(), msg.actions.clone())
                .map(
                    |(key, bind_type, action): (i32, i16, i32)| KeybindingModel {
                        id: None,
                        char_id: char.id,
                        key,
                        bind_type,
                        action,
                        created_at: None,
                        updated_at: SystemTime::now(),
                    },
                )
                .collect();
        command_tx
            .0
            .lock()
            .unwrap()
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::UpdateKeybindings {
                    client_id: msg.client_id,
                    binds,
                },
            ))
            .unwrap();
    }
}
