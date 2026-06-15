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

use crate::component::session::{InSession, MapleSession, Transitioning};
use crate::message::packet::raw::RawPacketMessage;
use crate::resource::custom_resource::{ClientMap, CustomReceiver};
use crate::system::event::RawEvent;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use ipc::event::AsyncEvent;
use std::sync::MutexGuard;
use std::sync::mpsc::Receiver;

pub fn handle_events_system(
    mut commands: Commands,
    receiver: Res<CustomReceiver>,
    mut client_map: ResMut<ClientMap>,
    transitioning: Query<Entity, With<Transitioning>>,
    mut raw_event_writer: MessageWriter<RawEvent>,
    mut raw_packet_writer: MessageWriter<RawPacketMessage>,
) {
    let rx: MutexGuard<Receiver<AsyncEvent>> = receiver.0.lock().unwrap();
    while let Ok(event) = rx.try_recv() {
        match event {
            AsyncEvent::ClientConnected {
                client_id,
                client_addr,
            } => {
                if let None = client_map.0.get(&client_id) {
                    let session_entity = commands.spawn(MapleSession { client_addr }).id();
                    client_map.0.insert(client_id, session_entity);
                    let Some(&client_entity) = client_map.0.get(&client_id) else {
                        continue;
                    };
                    commands
                        .entity(client_entity)
                        .insert(InSession(session_entity));
                }
            }
            AsyncEvent::ClientDisconnected { client_id } => {
                if let Some(client_entity) = client_map.0.get(&client_id).copied() {
                    if transitioning.get(client_entity).is_ok() {
                        continue;
                    }
                    client_map.0.remove(&client_id);
                    commands.entity(client_entity).despawn();
                }
            }
            AsyncEvent::PacketReceived { client_id, packet } => {
                raw_packet_writer.write(RawPacketMessage { client_id, packet });
            }
            AsyncEvent::LoginValid { .. } => {
                raw_event_writer.write(RawEvent::LoginValid(event));
            }
            AsyncEvent::LoginInvalid { .. } => {
                raw_event_writer.write(RawEvent::LoginInvalid(event));
            }
            AsyncEvent::SelectCharWithPic { .. } => {
                raw_event_writer.write(RawEvent::SelectCharWithPic(event));
            }
            AsyncEvent::ListCharsSuccess { .. } => {
                raw_event_writer.write(RawEvent::ListCharsSuccess(event));
            }
            AsyncEvent::ListCharsFailed { .. } => {
                raw_event_writer.write(RawEvent::ListCharsFailed(event));
            }
            AsyncEvent::CharCreationSuccess { .. } => {
                raw_event_writer.write(RawEvent::CharCreationSuccess(event));
            }
            AsyncEvent::CheckCharName { .. } => {
                raw_event_writer.write(RawEvent::CheckCharName(event));
            }
            AsyncEvent::JoinSuccess { .. } => {
                raw_event_writer.write(RawEvent::JoinSuccess(event));
            }
            AsyncEvent::CloseAttackSuccess { .. } => {
                raw_event_writer.write(RawEvent::CloseAttackSuccess(event));
            }
            AsyncEvent::PickupSuccess { .. } => {
                raw_event_writer.write(RawEvent::PickupSuccess(event));
            }
            AsyncEvent::ChangeMapSuccess { .. } => {
                raw_event_writer.write(RawEvent::ChangeMapSuccess(event));
            }
            AsyncEvent::DeadMobSuccess { .. } => {
                raw_event_writer.write(RawEvent::DeadMobSuccess(event));
            }
            _ => {}
        }
    }
}
