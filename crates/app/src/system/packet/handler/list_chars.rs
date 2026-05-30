/* list_chars/store.rs
 * The purpose of this module is to resolve relevant variables for character listing.
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

use crate::component::channel::InChannel;
use crate::component::session::MapleSession;
use crate::component::world::InWorld;
use crate::message::packet::list_chars::CharSlotsLoadedMessage;
use crate::resource::custom_resource::CustomSender;
use crate::system::packet::build::list_chars;
use crate::system::packet::handler::result::HandlerResult;
use crate::{component::account::MapleAccount, message::packet::list_chars::ListCharsMessage};
use bevy::ecs::message::MessageReader;
use bevy::ecs::system::{Commands, Query, Res};
use config::settings;
use ipc::tcp_command::TcpCommand;

pub enum PicStatus {
    Disabled = 2,
    AlreadyRegistered = 1,
    NeedsToRegister = 0,
}

pub async fn handle_load_char_slots(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<ListCharsRequestMessage>,
    command_tx: CustomSender<TcpCommand>,
    accounts: Query<&MapleAccount>,
) -> () {
    for msg in messages.read() {
        let Some(client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Some(acc) = accounts.get(client_entity) else {
            continue;
        };

        command_tx
            .0
            .send(TcpCommand::ListChars((msg, acc.id).into()))
            .unwrap();
    }
}

pub async fn handle_list_chars(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    mut messages: MessageReader<ListCharsSuccessMessage>,
    mut results: MessageWriter<HandlerResult>,
    accounts: Query<&MapleAccount>,
) -> () {
    for msg in messages.read() {
        let Some(client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        for char in &msg.chars {
            commands.spawn((MapleCharacter::from(char), ChildOf(client_entity)));
        }
        let Some(acc) = accounts.get(client_entity) else {
            continue;
        };

        let mut pic_status: i16 = PicStatus::Disabled as i16;
        let Some(use_pic) = settings::get_pic_required() else {
            continue;
        };
        if acc.pic.clone().is_some() {
            if use_pic {
                pic_status = PicStatus::AlreadyRegistered as i16;
            }
        } else {
            pic_status = PicStatus::NeedsToRegister as i16;
        };

        let Ok(list_chars_packet) =
            list_chars::build_list_chars_packet(msg.chars, msg.channel_id, msg.slots, pic_status)
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Session(SessionAction::Send {
                packet: list_chars_packet.finish(),
                scope: SessionScope::Local,
            })],
        });
    }
}
