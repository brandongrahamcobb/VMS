/* delete_char/store.rs
 * The purpose of this module is to resolve relevant variables for character deletion.
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

use crate::component::account::MapleAccount;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use bevy::ecs::message::MessageReader;
use bevy::ecs::system::{Query, Res};
use config::settings;
use ipc::tcp_command::AsyncCommand;

pub async fn handle_delete_char_request(
    client_map: Res<ClientMap>,
    commands: Commands,
    mut messages: MessageReader<DeleteCharRequestMessage>,
    command_tx: CustomSender<AsyncCommand>,
    accounts: Query<&MapleAccount>,
) -> () {
    for msg in messages.read() {
        let mut pic_status = false;
        let use_pic = settings::get_pic_required() else {
            continue;
        };

        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(acc) = accounts.get(client_entity) else {
            continue;
        };

        if use_pic {
            pic_status = domain::account::check_pic(acc.pic, msg.pic.clone());
        }
        if !pic_status {
            commands_tx.0.send(AsyncCommand::DeleteChar {
                client_id: msg.clinet_id,
                char_id: msg.char_id,
            });
            let Ok(spw_packet): Option<Packet> = spw::build_spw_packet(!pic_status) else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::Session(SessionAction::Send {
                    packet: spw_packet.finish(),
                    scope: SessionScope::Local,
                })],
            });
        }
    }
}
