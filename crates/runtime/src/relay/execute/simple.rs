/* simple.rs
 * The purpose of this module is to provide common relay handling.
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
use action::scope::{ChannelScope, MapScope};
use packet::model::Packet;
use session::model::Session;
use state::model::SharedState;

pub async fn simply_send_locally(
    _state: &SharedState,
    session: &Session,
    packet: Packet,
) -> Result<(), ExecuteError> {
    session.tx.send(packet.clone())?;
    Ok(())
}

pub async fn simply_send_to_map(
    state: &SharedState,
    session: &Session,
    packet: Packet,
    map_scope: MapScope,
) -> Result<(), ExecuteError> {
    match map_scope {
        MapScope::SameChannelSameWorld => {
            let sessions = {
                let locked_state = state.lock().await;
                locked_state.sessions.get_by_map_channel_world(
                    session.get_map_wz()?,
                    session.get_channel_id()?,
                    session.get_world_id()?,
                    session.id,
                )
            };
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
        MapScope::AllChannelsSameWorld => {
            let sessions = {
                let locked_state = state.lock().await;
                locked_state.sessions.get_by_map_world(
                    session.get_map_wz()?,
                    session.get_world_id()?,
                    session.id,
                )
            };
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
        MapScope::AllChannelsAllWorlds => {
            let sessions = {
                let locked_state = state.lock().await;
                locked_state
                    .sessions
                    .get_by_map(session.get_map_wz()?, session.id)
            };
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
    }
    Ok(())
}

pub async fn simply_send_to_channel(
    state: &SharedState,
    session: &Session,
    packet: Packet,
    channel_scope: ChannelScope,
) -> Result<(), ExecuteError> {
    match channel_scope {
        ChannelScope::SameWorld => {
            let sessions = {
                let locked_state = state.lock().await;
                locked_state.sessions.get_by_channel_world(
                    session.get_channel_id()?,
                    session.get_world_id()?,
                    session.id,
                )
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
                    .get_by_channel(session.get_channel_id()?, session.id)
            };
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
    }
    Ok(())
}

pub async fn simply_send_to_world(
    state: &SharedState,
    session: &Session,
    packet: Packet,
) -> Result<(), ExecuteError> {
    let sessions = {
        let locked_state = state.lock().await;
        locked_state
            .sessions
            .get_by_world(session.get_world_id()?, session.id)
    };
    for s in sessions {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}

pub async fn simply_send_globally(
    state: &SharedState,
    session: &Session,
    packet: Packet,
) -> Result<(), ExecuteError> {
    let sessions = {
        let locked_state = state.lock().await;
        locked_state.sessions.get_all(session.id)
    };
    for s in sessions {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}
