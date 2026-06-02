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

use crate::component::account::InAccount;
use crate::component::channel::InChannel;
use crate::component::character::MapleCharacter;
use crate::component::world::{InWorld, MapleWorld};
use crate::message::packet::list_chars::{
    CharSlotsLoadedMessage, ListCharsRequestMessage, ListCharsSuccessMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::list_chars;
use crate::{component::account::MapleAccount, message::packet::list_chars::ListCharsMessage};
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use config::settings;
use ipc::asyncronous::db_command::DatabaseCommand;

pub enum PicStatus {
    Disabled = 2,
    AlreadyRegistered = 1,
    NeedsToRegister = 0,
}

pub fn handle_load_char_slots(
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    worlds: Query<&MapleWorld>,
    in_worlds: Query<(Entity, &InWorld)>,
    accounts: Query<&MapleAccount>,
    in_accounts: Query<(Entity, &InAccount)>,
    mut messages: MessageReader<ListCharsRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_accounts.get(client_entity) else {
            continue;
        };
        let Ok(acc) = accounts.get(in_acc_entity) else {
            continue;
        };
        let Ok((in_world_entity, _)) = in_worlds.get(client_entity) else {
            continue;
        };
        let Ok(world) = worlds.get(in_world_entity) else {
            continue;
        };

        command_tx
            .0
            .lock()
            .unwrap()
            .send(DatabaseCommand::ListChars((msg, acc.id, world.id).into()))
            .unwrap();
    }
}

pub fn handle_list_chars(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    accounts: Query<(Entity, &MapleAccount)>,
    in_accounts: Query<(Entity, &InAccount)>,
    mut messages: MessageReader<ListCharsSuccessMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((acc_entity, acc)) = accounts.get(in_acc_entity) else {
            continue;
        };
        for char_model in &msg.char_models {
            commands.spawn((MapleCharacter::from(char_model), ChildOf(acc_entity)));
        }

        let mut pic_status: i16 = PicStatus::Disabled as i16;
        let Ok(use_pic) = settings::get_pic_required() else {
            continue;
        };
        if acc.pic.clone().is_some() {
            if use_pic {
                pic_status = PicStatus::AlreadyRegistered as i16;
            }
        } else {
            pic_status = PicStatus::NeedsToRegister as i16;
        };

        let Ok(mut list_chars_packet) =
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
