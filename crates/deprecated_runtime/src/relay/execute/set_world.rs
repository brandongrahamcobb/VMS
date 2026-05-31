/* set_world.rs
 * The purpose of this module is to provide the set world functions for relay handling.
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
use session::model::Session;
use state::model::SharedState;

pub async fn set_world_locally(
    state: &SharedState,
    session: &Session,
    world_id: i16,
) -> Result<(), ExecuteError> {
    let locked_state = state.lock().await;
    locked_state.sessions.update(session.id, |s| {
        s.world_id = Some(world_id);
    });
    Ok(())
}

pub async fn set_world_for_map(
    state: &SharedState,
    session: &Session,
    map_scope: MapScope,
    world_id: i16,
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
                let locked_state = state.lock().await;
                locked_state.sessions.update(s.id, |s| {
                    s.world_id = Some(world_id);
                });
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
                let locked_state = state.lock().await;
                locked_state.sessions.update(s.id, |s| {
                    s.world_id = Some(world_id);
                });
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
                let locked_state = state.lock().await;
                locked_state.sessions.update(s.id, |s| {
                    s.world_id = Some(world_id);
                });
            }
        }
    }
    Ok(())
}

pub async fn set_world_for_channel(
    state: &SharedState,
    session: &Session,
    channel_scope: ChannelScope,
    world_id: i16,
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
                let locked_state = state.lock().await;
                locked_state.sessions.update(s.id, |s| {
                    s.world_id = Some(world_id);
                });
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
                let locked_state = state.lock().await;
                locked_state.sessions.update(s.id, |s| {
                    s.world_id = Some(world_id);
                });
            }
        }
    }
    Ok(())
}

pub async fn set_world_for_world(
    state: &SharedState,
    session: &Session,
    world_id: i16,
) -> Result<(), ExecuteError> {
    let sessions = {
        let locked_state = state.lock().await;
        locked_state
            .sessions
            .get_by_world(session.get_world_id()?, session.id)
    };
    for s in sessions {
        let locked_state = state.lock().await;
        locked_state.sessions.update(s.id, |s| {
            s.world_id = Some(world_id);
        });
    }
    Ok(())
}

pub async fn set_world_globally(
    state: &SharedState,
    session: &Session,
    world_id: i16,
) -> Result<(), ExecuteError> {
    let sessions = {
        let locked_state = state.lock().await;
        locked_state.sessions.get_all(session.id)
    };
    for s in sessions {
        let locked_state = state.lock().await;
        locked_state.sessions.update(s.id, |s| {
            s.world_id = Some(world_id);
        });
    }
    Ok(())
}
