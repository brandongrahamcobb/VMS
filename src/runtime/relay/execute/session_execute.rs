/* execute/manager.rs
 * The purpose of this module is to provide action-wide functions for relay handling.
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

use tokio::sync::broadcast;

use crate::models::map::model::MapModel;
use crate::net::packet::handler::mob_respawn;
use crate::net::packet::handler::mob_respawn::handler::MobRespawnHandler;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::execute::error::ExecuteError;
use crate::runtime::relay::execute::{set_channel, set_map, set_world, simple};
use crate::runtime::relay::scope::SessionScope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use core::ops::ControlFlow;

pub async fn end(
    state: &SharedState,
    session: &Session,
    packet: &Packet,
    scope: &SessionScope,
) -> Result<ControlFlow<Packet>, ExecuteError> {
    match scope {
        SessionScope::Local => {
            return Ok(ControlFlow::Break(packet.clone()));
        }
        SessionScope::Map(map_scope) => {
            simple::simply_send_to_map(state, session, packet, map_scope).await?
        }
        SessionScope::Channel(channel_scope) => {
            simple::simply_send_to_channel(state, session, packet, channel_scope).await?
        }
        SessionScope::World => simple::simply_send_to_world(state, session, packet).await?,
        SessionScope::Global => simple::simply_send_globally(state, session, packet).await?,
    }
    Ok(ControlFlow::Continue(()))
}

pub async fn send(
    state: &SharedState,
    session: &Session,
    packet: &Packet,
    scope: &SessionScope,
) -> Result<ControlFlow<Packet>, ExecuteError> {
    match scope {
        SessionScope::Local => {
            session.tx.send(packet.clone())?;
        }
        SessionScope::Map(map_scope) => {
            simple::simply_send_to_map(state, session, packet, map_scope).await?
        }
        SessionScope::Channel(channel_scope) => {
            simple::simply_send_to_channel(state, session, packet, channel_scope).await?
        }
        SessionScope::World => simple::simply_send_to_world(state, session, packet).await?,
        SessionScope::Global => simple::simply_send_globally(state, session, packet).await?,
    }
    Ok(ControlFlow::Continue(()))
}

pub async fn retrieve(state: &SharedState, session: &Session) -> Result<(), ExecuteError> {
    let world_id: i16 = session.get_world_id()?;
    let channel_id: u8 = session.get_channel_id()?;
    let map_wz: i32 = session.get_map_wz()?;
    let packets: Vec<Packet> = {
        let state = state.lock().await;
        state
            .with_map(
                world_id,
                channel_id,
                map_wz,
                |map| -> Result<Vec<Packet>, ExecuteError> {
                    let packets: Vec<Packet> = {
                        let mut packets: Vec<Packet> = Vec::<Packet>::new();
                        for player in map.chars.values() {
                            packets.push(
                                Packet::new_empty()
                                    .build_spawn_player_packet(player)?
                                    .finish(),
                            );
                        }
                        for mob in map.mobs.values() {
                            packets.push(Packet::new_empty().build_spawn_mob_packet(mob)?.finish());
                        }
                        packets
                    };
                    Ok(packets)
                },
            )
            .await??
    };
    for packet in packets {
        session.tx.send(packet)?;
    }
    Ok(())
}

pub async fn set_map(
    state: &SharedState,
    session: &Session,
    scope: &SessionScope,
    map_wz: i32,
) -> Result<broadcast::Receiver<HandlerResult>, ExecuteError> {
    let world_id: i16 = session.get_world_id()?;
    let channel_id: u8 = session.get_channel_id()?;
    let tick_rx = insert_map(state, world_id, channel_id, map_wz).await?;
    match scope {
        SessionScope::Local => {
            set_map::set_map_locally(state, session, map_wz).await?;
        }
        SessionScope::Map(map_scope) => {
            set_map::set_map_for_map(state, session, map_scope, map_wz).await?
        }
        SessionScope::Channel(channel_scope) => {
            set_map::set_map_for_channel(state, session, channel_scope, map_wz).await?
        }
        SessionScope::World => set_map::set_map_for_world(state, session, map_wz).await?,
        SessionScope::Global => set_map::set_map_globally(state, session, map_wz).await?,
    }
    Ok(tick_rx)
}

pub async fn set_channel(
    state: &SharedState,
    session: &Session,
    scope: &SessionScope,
    channel_id: u8,
) -> Result<(), ExecuteError> {
    match scope {
        SessionScope::Local => {
            set_channel::set_channel_locally(state, session, channel_id).await?;
        }
        SessionScope::Map(map_scope) => {
            set_channel::set_channel_for_map(state, session, map_scope, channel_id).await?
        }
        SessionScope::Channel(channel_scope) => {
            set_channel::set_channel_for_channel(state, session, channel_scope, channel_id).await?
        }
        SessionScope::World => {
            set_channel::set_channel_for_world(state, session, channel_id).await?
        }
        SessionScope::Global => {
            set_channel::set_channel_globally(state, session, channel_id).await?
        }
    }
    Ok(())
}

pub async fn set_world(
    state: &SharedState,
    session: &Session,
    scope: &SessionScope,
    world_id: i16,
) -> Result<(), ExecuteError> {
    match scope {
        SessionScope::Local => {
            set_world::set_world_locally(state, session, world_id).await?;
        }
        SessionScope::Map(map_scope) => {
            set_world::set_world_for_map(state, session, map_scope, world_id).await?
        }
        SessionScope::Channel(channel_scope) => {
            set_world::set_world_for_channel(state, session, channel_scope, world_id).await?
        }
        SessionScope::World => set_world::set_world_for_world(state, session, world_id).await?,
        SessionScope::Global => set_world::set_world_globally(state, session, world_id).await?,
    }
    Ok(())
}

pub async fn set_acc(
    state: &SharedState,
    session: &Session,
    acc_id: i32,
) -> Result<(), ExecuteError> {
    let state = state.lock().await;
    state.sessions.update(session.id, |s| {
        s.acc_id = Some(acc_id);
    });
    Ok(())
}

pub async fn set_char(
    state: &SharedState,
    session: &Session,
    char_id: i32,
) -> Result<(), ExecuteError> {
    let state = state.lock().await;
    state.sessions.update(session.id, |s| {
        s.char_id = Some(char_id);
    });
    Ok(())
}

pub async fn insert_map(
    state: &SharedState,
    world_id: i16,
    channel_id: u8,
    map_wz: i32,
) -> Result<broadcast::Receiver<HandlerResult>, ExecuteError> {
    let map_model: MapModel = MapModel { wz: map_wz };
    let map = map_model.load(state, world_id, channel_id, map_wz).await?;
    let state = state.lock().await;
    let exists = state
        .with_channel(world_id, channel_id, |channel| {
            if channel.maps.contains_key(&map_wz) {
                true
            } else {
                false
            }
        })
        .await?;
    if !exists {
        state
            .with_mut_channel(world_id, channel_id, |channel| {
                channel.maps.insert(map_wz, map);
            })
            .await?;
    }
    let tick_rx = state
        .with_channel(world_id, channel_id, |channel| {
            channel.maps.get(&map_wz).unwrap().tick_tx.subscribe()
        })
        .await?;
    Ok(tick_rx)
}

pub async fn delete_map(
    state: &SharedState,
    world_id: i16,
    channel_id: u8,
    map_wz: i32,
) -> Result<(), ExecuteError> {
    let state = state.lock().await;
    state
        .with_mut_channel(world_id, channel_id, |channel| {
            channel.maps.remove(&map_wz);
        })
        .await?;
    Ok(())
}
