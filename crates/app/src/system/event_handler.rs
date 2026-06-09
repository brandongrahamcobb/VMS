/* app/src/system/event_handler.rs
 * The purpose of this module is to provide a system for handling plugin events.
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

use crate::component::session::MapleSession;
use crate::message::packet::attack_close::CloseAttackResponseMessage;
use crate::message::packet::check_char_name::CheckCharNameResponseMessage;
use crate::message::packet::create_char::CreateCharResponseMessage;
use crate::message::packet::list_chars::{
    ListCharsFailedResponseMessage, ListCharsSuccessResponseMessage,
};
use crate::message::packet::login::{LoginFailedResponseMessage, LoginSuccessResponseMessage};
use crate::message::packet::pickup_item::PickupItemResponseMessage;
use crate::message::packet::player_logged_in::PlayerLoggedInResponseMessage;
use crate::message::packet::raw::RawPacketMessage;
use crate::resource::custom_resource::{ClientMap, CustomReceiver};
use bevy::ecs::hierarchy::Children;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::{Commands, Res, ResMut};
use ipc::asyncronous::event::AsyncEvent;
use std::sync::MutexGuard;
use std::sync::mpsc::Receiver;

pub fn handle_events_system(
    mut commands: Commands,
    receiver: Res<CustomReceiver>,
    mut client_map: ResMut<ClientMap>,
    mut check_char_name_response_writer: MessageWriter<CheckCharNameResponseMessage>,
    mut create_char_response_writer: MessageWriter<CreateCharResponseMessage>,
    mut list_chars_success_writer: MessageWriter<ListCharsSuccessResponseMessage>,
    mut list_chars_fail_writer: MessageWriter<ListCharsFailedResponseMessage>,
    mut player_join_success_writer: MessageWriter<PlayerLoggedInResponseMessage>,
    mut pickup_success_writer: MessageWriter<PickupItemResponseMessage>,
    mut close_attack_success_writer: MessageWriter<CloseAttackResponseMessage>,
    mut raw_packet_writer: MessageWriter<RawPacketMessage>,
    mut login_success_writer: MessageWriter<LoginSuccessResponseMessage>,
    mut login_fail_writer: MessageWriter<LoginFailedResponseMessage>,
) {
    let rx: MutexGuard<Receiver<AsyncEvent>> = receiver.0.lock().unwrap();
    while let Ok(event) = rx.try_recv() {
        match event {
            AsyncEvent::ClientConnected { client_id } => {
                let session_entity = commands
                    .spawn(MapleSession {
                        transitioning: false,
                    })
                    .id();
                client_map.0.insert(client_id, session_entity);
            }
            AsyncEvent::ClientDisconnected { client_id } => {
                if let Some(client_entity) = client_map.0.remove(&client_id) {
                    commands.entity(client_entity).despawn_related::<Children>();
                }
            }
            AsyncEvent::PacketReceived { client_id, packet } => {
                raw_packet_writer.write(RawPacketMessage { client_id, packet });
            }
            AsyncEvent::LoginSuccess {
                client_id,
                acc_id,
                acc_model,
            } => {
                login_success_writer.write(LoginSuccessResponseMessage {
                    client_id,
                    acc_id,
                    acc_model,
                });
            }
            AsyncEvent::LoginFailed { client_id, code } => {
                login_fail_writer.write(LoginFailedResponseMessage { client_id, code });
            }

            AsyncEvent::ListCharsSuccess {
                client_id,
                channel_id,
                char_models,
                keybinding_model_map,
                skill_model_map,
                equipped_item_model_map,
                equip_item_model_map,
                use_item_model_map,
                etc_item_model_map,
                setup_item_model_map,
                cash_item_model_map,
                equip_tab_inv_capacity_map,
                use_tab_inv_capacity_map,
                etc_tab_inv_capacity_map,
                setup_tab_inv_capacity_map,
                cash_tab_inv_capacity_map,
                slots,
                world_id,
            } => {
                list_chars_success_writer.write(ListCharsSuccessResponseMessage {
                    client_id,
                    channel_id,
                    char_models,
                    keybinding_model_map,
                    skill_model_map,
                    equipped_item_model_map,
                    equip_item_model_map,
                    use_item_model_map,
                    etc_item_model_map,
                    setup_item_model_map,
                    cash_item_model_map,
                    equip_tab_inv_capacity_map,
                    use_tab_inv_capacity_map,
                    etc_tab_inv_capacity_map,
                    setup_tab_inv_capacity_map,
                    cash_tab_inv_capacity_map,
                    slots,
                    world_id,
                });
            }
            AsyncEvent::ListCharsFailed { client_id } => {
                list_chars_fail_writer.write(ListCharsFailedResponseMessage { client_id });
            }
            AsyncEvent::CharCreationSuccess {
                client_id,
                char_model,
                equipped_item_model_map,
                equip_item_model_map,
                use_item_model_map,
                etc_item_model_map,
                setup_item_model_map,
                cash_item_model_map,
                keybinding_model_map,
                skill_model_map,
                equip_tab_inv_capacity_map,
                use_tab_inv_capacity_map,
                etc_tab_inv_capacity_map,
                setup_tab_inv_capacity_map,
                cash_tab_inv_capacity_map,
            } => {
                let Some(char_id) = char_model.id else {
                    continue;
                };
                create_char_response_writer.write(CreateCharResponseMessage {
                    client_id,
                    char_id,
                    char_model,
                    keybinding_model_map,
                    skill_model_map,
                    equipped_item_model_map,
                    equip_item_model_map,
                    use_item_model_map,
                    etc_item_model_map,
                    setup_item_model_map,
                    cash_item_model_map,
                    equip_tab_inv_capacity_map,
                    use_tab_inv_capacity_map,
                    etc_tab_inv_capacity_map,
                    setup_tab_inv_capacity_map,
                    cash_tab_inv_capacity_map,
                });
            }
            AsyncEvent::CheckCharName {
                client_id,
                exists,
                ign,
            } => {
                check_char_name_response_writer.write(CheckCharNameResponseMessage {
                    client_id,
                    exists,
                    ign,
                });
            }
            AsyncEvent::JoinSuccess {
                client_id,
                char_id,
                map_wz,
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
                    client_id,
                    char_id,
                    map_wz,
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
                });
            }
            AsyncEvent::CloseAttackSuccess {
                client_id,
                count,
                skill_model,
                base_skill,
                display,
                toleft,
                stance,
                speed,
                mob_damages,
            } => {
                close_attack_success_writer.write(CloseAttackResponseMessage {
                    client_id,
                    count,
                    skill_model,
                    base_skill,
                    display,
                    toleft,
                    stance,
                    speed,
                    mob_damages,
                });
            }
            AsyncEvent::PickupSuccess {
                client_id,
                item_id,
                ipos,
                pet_pickup,
            } => {
                pickup_success_writer.write(PickupItemResponseMessage {
                    client_id,
                    item_id,
                    ipos,
                    pet_pickup,
                });
            }
            _ => {}
        }
    }
}
