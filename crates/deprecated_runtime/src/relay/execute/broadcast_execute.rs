/* broadcast_execute.rs
 * The purpose of this module is to provide actions related to all clients.
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

use crate::relay::execute::error::ExecuteError;
use action::scope::BroadcastScope;
use net::packet::model::Packet;
use rand::seq::IndexedRandom;
use state::model::SharedState;

pub async fn broadcast(
    state: &SharedState,
    packet: Packet,
    scope: BroadcastScope,
) -> Result<(), ExecuteError> {
    match scope {
        BroadcastScope::Global => broadcast_globally(state, packet).await?,
        BroadcastScope::GlobalChar => broadcast_to_one_member_globally(state, packet).await?,
        BroadcastScope::World { world_id } => broadcast_to_world(state, packet, world_id).await?,
        BroadcastScope::WorldChar { world_id } => {
            broadcast_to_one_member_in_world(state, packet, world_id).await?
        }
        BroadcastScope::Channel {
            world_id,
            channel_id,
        } => broadcast_to_channel(state, packet, world_id, channel_id).await?,
        BroadcastScope::ChannelChar {
            world_id,
            channel_id,
        } => broadcast_to_one_member_in_channel(state, packet, world_id, channel_id).await?,
        BroadcastScope::Map {
            world_id,
            channel_id,
            map_wz,
        } => broadcast_to_map(state, packet, world_id, channel_id, map_wz).await?,
        BroadcastScope::MapChar {
            world_id,
            channel_id,
            map_wz,
        } => broadcast_to_one_member_in_map(state, packet, world_id, channel_id, map_wz).await?,
    }
    Ok(())
}

pub async fn broadcast_to_one_member_in_map(
    state: &SharedState,
    packet: Packet,
    world_id: i16,
    channel_id: u8,
    map_wz: i32,
) -> Result<(), ExecuteError> {
    let no_session_id = 0;
    let sessions = {
        let locked_state = state.lock().await;
        locked_state
            .sessions
            .get_by_map_channel_world(map_wz, channel_id, world_id, no_session_id)
    };
    if let Some(s) = sessions.choose(&mut rand::rng()) {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}

pub async fn broadcast_to_map(
    state: &SharedState,
    packet: Packet,
    world_id: i16,
    channel_id: u8,
    map_wz: i32,
) -> Result<(), ExecuteError> {
    let no_session_id = 0;
    let sessions = {
        let locked_state = state.lock().await;
        locked_state
            .sessions
            .get_by_map_channel_world(map_wz, channel_id, world_id, no_session_id)
    };
    for s in sessions {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}

pub async fn broadcast_to_one_member_in_channel(
    state: &SharedState,
    packet: Packet,
    world_id: i16,
    channel_id: u8,
) -> Result<(), ExecuteError> {
    let no_session_id = 0;
    let sessions = {
        let locked_state = state.lock().await;
        locked_state
            .sessions
            .get_by_channel_world(channel_id, world_id, no_session_id)
    };
    if let Some(s) = sessions.choose(&mut rand::rng()) {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}

pub async fn broadcast_to_channel(
    state: &SharedState,
    packet: Packet,
    world_id: i16,
    channel_id: u8,
) -> Result<(), ExecuteError> {
    let no_session_id = 0;
    let sessions = {
        let locked_state = state.lock().await;
        locked_state
            .sessions
            .get_by_channel_world(channel_id, world_id, no_session_id)
    };
    for s in sessions {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}

pub async fn broadcast_to_one_member_in_world(
    state: &SharedState,
    packet: Packet,
    world_id: i16,
) -> Result<(), ExecuteError> {
    let no_session_id = 0;
    let sessions = {
        let locked_state = state.lock().await;
        locked_state.sessions.get_by_world(world_id, no_session_id)
    };
    if let Some(s) = sessions.choose(&mut rand::rng()) {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}

pub async fn broadcast_to_world(
    state: &SharedState,
    packet: Packet,
    world_id: i16,
) -> Result<(), ExecuteError> {
    let no_session_id = 0;
    let sessions = {
        let locked_state = state.lock().await;
        locked_state.sessions.get_by_world(world_id, no_session_id)
    };
    for s in sessions {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}

pub async fn broadcast_to_one_member_globally(
    state: &SharedState,
    packet: Packet,
) -> Result<(), ExecuteError> {
    let no_session_id = 0;
    let sessions = {
        let locked_state = state.lock().await;
        locked_state.sessions.get_all(no_session_id)
    };
    if let Some(s) = sessions.choose(&mut rand::rng()) {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}

pub async fn broadcast_globally(state: &SharedState, packet: Packet) -> Result<(), ExecuteError> {
    let no_session_id = 0;
    let sessions = {
        let locked_state = state.lock().await;
        locked_state.sessions.get_all(no_session_id)
    };
    for s in sessions {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}
