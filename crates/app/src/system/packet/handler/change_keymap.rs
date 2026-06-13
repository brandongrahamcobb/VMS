/* app/src/system/handler/change_keymap.rs
 * The purpose of this module is to handle changing keymap system messages.
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

use crate::message::packet::change_keymap::ReadChangeKeymapRequestMessage;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::system_params::{InParams, SessionParams};
use bevy::ecs::message::MessageReader;
use bevy::ecs::system::Res;
use db::keybinding::model::KeybindingModel;
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;
use itertools::izip;
use std::time::SystemTime;

pub fn handle_change_keymap(
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    mut messages: MessageReader<ReadChangeKeymapRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char.0) else {
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
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::UpdateKeybindings {
                    client_id: msg.client_id,
                    binds,
                },
            ))
            .unwrap();
    }
}
