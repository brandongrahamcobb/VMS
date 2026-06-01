/* register_pic/store.rs
 * The purpose of this module is to resolve relevant variables for PIC registration.
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

use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Query, Res};
use ipc::tcp_command::AsyncCommand;
use net::packet::model::Packet;

use crate::component::session::MapleSession;
use crate::message::packet::register_pic::RegisterPicMessage;
use crate::resource::custom_resource::{ClientMap, CustomSender, Sessions};
use crate::system::packet::build::spw;
use crate::system::packet::handler::result::HandlerResult;

pub async fn store_register_pic(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<RegisterPicMessage>,
    command_tx: CustomSender<AsyncCommand>,
    mut results: MessageWriter<HandlerResult>,
    accounts: Query<(Entity, &MapleAccount)>,
) -> () {
    for msg in messages.read() {
        let Some(client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(acc) = accounts.get(client_entity) else {
            continue;
        };

        command_tx
            .0
            .send(AsyncCommand::SetPic {
                client_id: msg.client_id,
                acc_id: acc.id,
                pic: msg.pic,
            })
            .unwrap();

        let success_status: bool = true;
        let Ok(spw_packet): Option<Packet> = spw::build_spw_packet(success_status) else {
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
