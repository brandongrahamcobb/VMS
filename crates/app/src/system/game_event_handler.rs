/* app/src/system/game_event_handler.rs
 * The purpose of this module is to provide a system for handling raw game events.
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

use crate::message::packet::pickup_item::PickupItemResponseMessage;
use crate::message::packet::player_logged_in::PlayerLoggedInResponseMessage;
use crate::message::packet::player_map_transferred::PlayerMapTransferResponseMessage;
use crate::system::event::RawEvent;
use bevy::ecs::message::{MessageReader, MessageWriter};
use ipc::event::AsyncEvent;

pub fn handle_game_events_system(
    mut messages: MessageReader<RawEvent>,
    mut player_join_success_writer: MessageWriter<PlayerLoggedInResponseMessage>,
    mut pickup_success_writer: MessageWriter<PickupItemResponseMessage>,
    mut player_map_transfer_success_writer: MessageWriter<PlayerMapTransferResponseMessage>,
) {
    for msg in messages.read() {
        match msg {
            RawEvent::JoinSuccess(event) => match event {
                AsyncEvent::JoinSuccess {
                    client_id,
                    char_id,
                    keybinding_models,
                    skill_models,
                    equipped_item_models,
                    equip_tab_item_models,
                    use_tab_item_models,
                    etc_tab_item_models,
                    setup_tab_item_models,
                    cash_tab_item_models,
                    equip_tab_capacity,
                    use_tab_capacity,
                    etc_tab_capacity,
                    setup_tab_capacity,
                    cash_tab_capacity,
                } => {
                    player_join_success_writer.write(PlayerLoggedInResponseMessage {
                        client_id: *client_id,
                        char_id: *char_id,
                        keybinding_models: keybinding_models.clone(),
                        skill_models: skill_models.clone(),
                        equipped_item_models: equipped_item_models.clone(),
                        equip_tab_item_models: equip_tab_item_models.clone(),
                        use_tab_item_models: use_tab_item_models.clone(),
                        etc_tab_item_models: etc_tab_item_models.clone(),
                        setup_tab_item_models: setup_tab_item_models.clone(),
                        cash_tab_item_models: cash_tab_item_models.clone(),
                        equip_tab_capacity: *equip_tab_capacity,
                        use_tab_capacity: *use_tab_capacity,
                        etc_tab_capacity: *etc_tab_capacity,
                        setup_tab_capacity: *setup_tab_capacity,
                        cash_tab_capacity: *cash_tab_capacity,
                    });
                }
                _ => {}
            },
            RawEvent::PickupSuccess(event) => match event {
                AsyncEvent::PickupSuccess {
                    client_id,
                    count,
                    item_id,
                    ipos,
                    itab,
                    pet_pickup,
                } => {
                    pickup_success_writer.write(PickupItemResponseMessage {
                        client_id: *client_id,
                        count: *count,
                        item_id: *item_id,
                        ipos: *ipos,
                        itab: itab.clone(),
                        pet_pickup: *pet_pickup,
                    });
                }
                _ => {}
            },
            RawEvent::ChangeMapSuccess(event) => match event {
                AsyncEvent::ChangeMapSuccess {
                    client_id,
                    base_map,
                    base_portals,
                    base_mobs,
                } => {
                    player_map_transfer_success_writer.write(PlayerMapTransferResponseMessage {
                        client_id: *client_id,
                        base_map: *base_map,
                        base_portals: base_portals.clone(),
                        base_mobs: base_mobs.clone(),
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }
}
