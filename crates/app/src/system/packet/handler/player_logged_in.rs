/* player_logged_in/store.rs
 * The purpose of this module is to resolve relevant variables for player login.
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
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::{codec, player_logged_in};
use crate::system::packet::handler::result::HandlerResult;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::{Query, Res};
use ipc::tcp_command::AsyncCommand;

pub async fn handle_player_logged_in_request(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<PlayerLoggedInRequestMessage>,
    command_tx: CustomSender<AsyncCommand>,
    mut results: MessageWriter<HandlerResult>,
    chars: Query<&MapleCharacter>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(char) = chars.get(client_entity) else {
            continue;
        };

        command_tx.0.send(AsyncCommand::JoinPlayer {
            client_id: msg.client_id,
        });
    }
}

pub async fn handle_player_logged_in_response(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<PlayerLoggedInResponseMessage>,
    command_tx: CustomSender<AsyncCommand>,
    mut results: MessageWriter<HandlerResult>,
    chars: Query<(Entity, &MapleCharacter)>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Some(char_entity, char) = chars.iter().find(|(_, c)| c.id == msg.char_id);

        commands.spawn((MapleKeybindings::from(binds), ChildOf(char)));
        commands.spawn((MapleKeybindings::from(binds), ChildOf(char)));
        commands.entity(client_entity).insert(InChar(char_entity));

        let Ok(keymap_packet) = player_logged_in::build_player_logged_in_keymap_packet(&msg.binds)
        else {
            continue;
        };
        let Ok(set_field_packet) =
            codec::player::builder::build_set_field_packet(&char, channel.id, char.map_wz)
        else {
            continue;
        };

        results.write(HandleResult {
            client_id: msg.client_id,
            actions: vec![
                Action::Session(SessionAction::Send {
                    packet: keymap_packet.finish(),
                    scope: SessionScope::Local,
                }),
                Action::Session(SessionAction::Send {
                    packet: packet.clone(),
                    scope: SessionScope::Local,
                }),
                Action::Session(SessionAction::Retrieve),
            ],
        });
    }
}

//     let mut char: Character =
//         assembly::character::assemble::assemble_char_by_id(pool, reader.char_id).await?;
//     let map_wz = char.model.map_wz;
//     let mut binds: HashMap<i32, Keybinding> =
//         assembly::keybinding::assemble::assemble_keybindings_by_char_id(pool, reader.char_id)
//             .await?;
//     for (key, bind) in char.binds.drain() {
//         binds.insert(key, bind);
//     }
//     Ok(Self {
//         binds,
//         channel_id: reader.channel_id as u8,
//         char,
//         map_wz,
//     })
// }
