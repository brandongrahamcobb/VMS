/* app/src/system/packet/handler/pickup_item.rs
 * The purpose of this module is to process item pickup system messages.
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

use crate::component::item::{Lootable, MapleItem};
use crate::component::slot::{MapleEmptyItemSlot, MapleFilledItemSlot};
use crate::message::packet::pickup_item::{
    PickupItemResponseMessage, ReadPickupItemRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::handler::result::pickup_loot_result;
use crate::system::system_params::{InParams, InventoryParams, SessionParams};
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
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
    items: Query<(Entity, &MapleItem)>,
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
        let Some((item_entity, _)) = items.iter().find(|(_, i)| i.id == msg.item_id) else {
            continue;
        };
        commands.entity(item_entity).remove::<Lootable>();
        // commands
        //     .entity(empty_slot_entity)
        //     .remove::<MapleEmptyItemSlot>();
        // commands.spawn(
        //     MapleFilledItemSlot {
        //         ipos: empty_slot.ipos,
        //     })
        //
        // commands
        //     .entity(item_entity)
        //     .remove::<Lootable>();
        //     .insert(ChildOf(MapleFilledItemSlot {
        //         ipos: empty_slot.ipos,
        //     }));

        command_tx
            .0
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::PickupItem {
                    client_id: msg.client_id,
                    char_id: char.id,
                    item_id: msg.item_id,
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
    inv_params: InventoryParams,
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
        let Some((inv_entity, inv, _)) = inv_params
            .inventories
            .iter()
            .find(|(_, _, parent)| parent.0 == in_char.0)
        else {
            continue;
        };
        let Some((empty_slot_entity, empty_slot, _)) = inv_params
            .empty_slots
            .iter()
            .filter(|(_, empty_slot, parent)| parent.0 == inv_entity && empty_slot)
            .next()
        else {
            continue;
        };

        pickup_loot_result::write_result(
            msg.client_id,
            &char.clone(),
            msg.item_id,
            msg.pet_pickup,
            &mut results,
        );
    }
}
