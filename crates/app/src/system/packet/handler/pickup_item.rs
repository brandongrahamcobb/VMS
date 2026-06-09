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

use crate::component::slot::{MapleEmptyItemSlot, MapleFilledItemSlot};
use crate::message::packet::pickup_item::{
    PickupItemResponseMessage, ReadPickupItemRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::pickup_item;
use crate::system::system_params::{InParams, SessionParams};
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;

pub fn handle_pickup_item_request(
    mut commands: Commands,
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    empty_slots: Query<(Entity, &MapleEmptyItemSlot)>,
    mut messages: MessageReader<ReadPickupItemRequestMessage>,
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
        let Some((empty_slot_entity, empty_slot)) = empty_slots.iter().next() else {
            continue;
        };
        commands
            .entity(empty_slot_entity)
            .remove::<MapleEmptyItemSlot>()
            .insert(MapleFilledItemSlot {
                ipos: empty_slot.ipos,
            });

        command_tx
            .0
            .lock()
            .unwrap()
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::PickupItem {
                    client_id: msg.client_id,
                    char_id: char.id,
                    item_id: msg.item_id,
                    ipos: empty_slot.ipos,
                    pet_pickup: msg.pet_pickup,
                },
            ))
            .unwrap();
    }
}

pub fn handle_pickup_response(
    client_map: Res<ClientMap>,
    session_params: SessionParams,
    in_params: InParams,
    mut messages: MessageReader<PickupItemResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
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

        let Ok(mut pickup_item_packet) =
            pickup_item::build_pickup_item_packet(char.id, msg.item_id, msg.pet_pickup)
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::HandlerAction {
                packet: pickup_item_packet.finish(),
                scope: ActionScope::Local,
            }],
        });
    }
}
