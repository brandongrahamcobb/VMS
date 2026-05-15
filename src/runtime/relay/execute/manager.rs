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

use crate::net::packet::model::Packet;
use crate::runtime::error::RuntimeError;
use crate::runtime::relay::execute::{set_channel, set_map, set_world, simple};
use crate::runtime::relay::scope::Scope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use core::ops::ControlFlow;

pub async fn end(
    state: &SharedState,
    session: &Session,
    packet: &Packet,
    scope: &Scope,
) -> Result<ControlFlow<Packet>, RuntimeError> {
    match scope {
        Scope::Local => {
            return Ok(ControlFlow::Break(packet.clone()));
        }
        Scope::Map(map_scope) => {
            simple::simply_send_to_map(state, session, packet, map_scope).await?
        }
        Scope::Channel(channel_scope) => {
            simple::simply_send_to_channel(state, session, packet, channel_scope).await?
        }
        Scope::World => simple::simply_send_to_world(state, session, packet).await?,
        Scope::Global => simple::simply_send_globally(state, session, packet).await?,
    }
    Ok(ControlFlow::Continue(()))
}

pub async fn send(
    state: &SharedState,
    session: &Session,
    packet: &Packet,
    scope: &Scope,
) -> Result<(), RuntimeError> {
    match scope {
        Scope::Local => {
            session.tx.send(packet.clone())?;
        }
        Scope::Map(map_scope) => {
            simple::simply_send_to_map(state, session, packet, map_scope).await?
        }
        Scope::Channel(channel_scope) => {
            simple::simply_send_to_channel(state, session, packet, channel_scope).await?
        }
        Scope::World => simple::simply_send_to_world(state, session, packet).await?,
        Scope::Global => simple::simply_send_globally(state, session, packet).await?,
    }
    Ok(())
}

pub async fn set_map(
    state: &SharedState,
    session: &Session,
    scope: &Scope,
    map_wz: i32,
) -> Result<(), RuntimeError> {
    match scope {
        Scope::Local => {
            set_map::set_map_locally(state, session, map_wz).await?;
        }
        Scope::Map(map_scope) => {
            set_map::set_map_for_map(state, session, map_scope, map_wz).await?
        }
        Scope::Channel(channel_scope) => {
            set_map::set_map_for_channel(state, session, channel_scope, map_wz).await?
        }
        Scope::World => set_map::set_map_for_world(state, session, map_wz).await?,
        Scope::Global => set_map::set_map_globally(state, session, map_wz).await?,
    }
    Ok(())
}

pub async fn set_channel(
    state: &SharedState,
    session: &Session,
    scope: &Scope,
    channel_id: u8,
) -> Result<(), RuntimeError> {
    match scope {
        Scope::Local => {
            set_channel::set_channel_locally(state, session, channel_id).await?;
        }
        Scope::Map(map_scope) => {
            set_channel::set_channel_for_map(state, session, map_scope, channel_id).await?
        }
        Scope::Channel(channel_scope) => {
            set_channel::set_channel_for_channel(state, session, channel_scope, channel_id).await?
        }
        Scope::World => set_channel::set_channel_for_world(state, session, channel_id).await?,
        Scope::Global => set_channel::set_channel_globally(state, session, channel_id).await?,
    }
    Ok(())
}

pub async fn set_world(
    state: &SharedState,
    session: &Session,
    scope: &Scope,
    world_id: i16,
) -> Result<(), RuntimeError> {
    match scope {
        Scope::Local => {
            set_world::set_world_locally(state, session, world_id).await?;
        }
        Scope::Map(map_scope) => {
            set_world::set_world_for_map(state, session, map_scope, world_id).await?
        }
        Scope::Channel(channel_scope) => {
            set_world::set_world_for_channel(state, session, channel_scope, world_id).await?
        }
        Scope::World => set_world::set_world_for_world(state, session, world_id).await?,
        Scope::Global => set_world::set_world_globally(state, session, world_id).await?,
    }
    Ok(())
}

pub async fn set_acc(
    state: &SharedState,
    session: &Session,
    acc_id: i32,
) -> Result<(), RuntimeError> {
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
) -> Result<(), RuntimeError> {
    let state = state.lock().await;
    state.sessions.update(session.id, |s| {
        s.char_id = Some(char_id);
    });
    Ok(())
}
