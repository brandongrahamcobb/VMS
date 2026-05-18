/* broadcast.rs
 * The purpose of this module is to provide broadcast relay handling.
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

use crate::net::packet::model::Packet;
use crate::runtime::relay::execute::error::ExecuteError;
use crate::runtime::relay::scope::{ChannelScope, MapScope};
use crate::runtime::state::SharedState;

pub async fn broadcast_to_map(
    state: &SharedState,
    packet: &Packet,
    world_id: i16,
    channel_id: u8,
    map_wz: i32,
    map_scope: &MapScope,
) -> Result<(), ExecuteError> {
    let no_session_id = 0;
    match map_scope {
        MapScope::SameChannelSameWorld => {
            let sessions = {
                let locked_state = state.lock().await;
                locked_state.sessions.get_by_map_channel_world(
                    map_wz,
                    channel_id,
                    world_id,
                    no_session_id,
                )
            };
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
        MapScope::AllChannelsSameWorld => {
            let sessions = {
                let locked_state = state.lock().await;
                locked_state
                    .sessions
                    .get_by_map_world(map_wz, world_id, no_session_id)
            };
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
        MapScope::AllChannelsAllWorlds => {
            let sessions = {
                let locked_state = state.lock().await;
                locked_state.sessions.get_by_map(map_wz, no_session_id)
            };
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
    }
    Ok(())
}

pub async fn broadcast_to_channel(
    state: &SharedState,
    packet: &Packet,
    world_id: i16,
    channel_id: u8,
    channel_scope: &ChannelScope,
) -> Result<(), ExecuteError> {
    let no_session_id = 0;
    match channel_scope {
        ChannelScope::SameWorld => {
            let sessions = {
                let locked_state = state.lock().await;
                locked_state
                    .sessions
                    .get_by_channel_world(channel_id, world_id, no_session_id)
            };
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
        ChannelScope::AllWorlds => {
            let sessions = {
                let locked_state = state.lock().await;
                locked_state
                    .sessions
                    .get_by_channel(channel_id, no_session_id)
            };
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
    }
    Ok(())
}

pub async fn broadcast_to_world(
    state: &SharedState,
    packet: &Packet,
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

pub async fn broadcast_globally(state: &SharedState, packet: &Packet) -> Result<(), ExecuteError> {
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
