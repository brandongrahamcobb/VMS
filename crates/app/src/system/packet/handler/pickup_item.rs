/* pickup_item/store.rs
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
use crate::component::slot::MapleEmptyItemSlot;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::pickup_item;
use crate::system::packet::handler::result::HandlerResult;
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res};
use ipc::asyncronous::db_command::DatabaseCommand;

pub fn handle_pickup_item_request(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<PickupItemRequestMessage>,
    command_tx: CustomSender,
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

        command_tx.0.send(DatabaseCommand::PickupItem {
            client_id: msg.client_id,
            char_id: char.id,
            item_id: msg.item_id,
        });
    }
}

pub fn handle_pickup_response(
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    chars: Query<&MapleCharacter>,
    in_chars: Query<(Entity, &InChar)>,
    empty_slots: Query<Entity, With<MapleEmptyItemSlot>>,
    mut messages: MessageReader<PickupItemResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
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

        let ipos: MapleEmptyItemSlot = empty_slots.iter().next();

        let Ok(mut pickup_item_packet) =
            pickup_item::build_pickup_item_packet(char.id, msg.item_id, msg.pet_pickup, ipos)
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Session(SessionAction::Send {
                packet: pickup_item_packet.finish(),
                scope: SessionScope::Local,
            })],
        });
    }
}
