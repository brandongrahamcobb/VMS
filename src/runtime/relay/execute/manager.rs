use crate::models::account::model::Account;
use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::models::shroom::map::model::Map;
use crate::models::shroom::world::model::World;
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
    map: &Map,
    scope: &Scope,
) -> Result<(), RuntimeError> {
    match scope {
        Scope::Local => {
            set_map::set_map_locally(state, session, map).await?;
        }
        Scope::Map(map_scope) => set_map::set_map_for_map(state, session, map, map_scope).await?,
        Scope::Channel(channel_scope) => {
            set_map::set_map_for_channel(state, session, map, channel_scope).await?
        }
        Scope::World => set_map::set_map_for_world(state, session, map).await?,
        Scope::Global => set_map::set_map_globally(state, session, map).await?,
    }
    Ok(())
}

pub async fn set_channel(
    state: &SharedState,
    session: &Session,
    channel: &Channel,
    scope: &Scope,
) -> Result<(), RuntimeError> {
    match scope {
        Scope::Local => {
            set_channel::set_channel_locally(state, session, channel).await?;
        }
        Scope::Map(map_scope) => {
            set_channel::set_channel_for_map(state, session, channel, map_scope).await?
        }
        Scope::Channel(channel_scope) => {
            set_channel::set_channel_for_channel(state, session, channel, channel_scope).await?
        }
        Scope::World => set_channel::set_channel_for_world(state, session, channel).await?,
        Scope::Global => set_channel::set_channel_globally(state, session, channel).await?,
    }
    Ok(())
}

pub async fn set_world(
    state: &SharedState,
    session: &Session,
    world: &World,
    scope: &Scope,
) -> Result<(), RuntimeError> {
    match scope {
        Scope::Local => {
            set_world::set_world_locally(state, session, world).await?;
        }
        Scope::Map(map_scope) => {
            set_world::set_world_for_map(state, session, world, map_scope).await?
        }
        Scope::Channel(channel_scope) => {
            set_world::set_world_for_channel(state, session, world, channel_scope).await?
        }
        Scope::World => set_world::set_world_for_world(state, session, world).await?,
        Scope::Global => set_world::set_world_globally(state, session, world).await?,
    }
    Ok(())
}

pub async fn set_acc(
    state: &SharedState,
    session: &Session,
    acc: &Account,
) -> Result<(), RuntimeError> {
    let state = state.lock().await;
    state.sessions.update(session.id, |s| {
        s.acc = Some(acc.clone());
    });
    Ok(())
}

pub async fn set_char(
    state: &SharedState,
    session: &Session,
    char: &Character,
) -> Result<(), RuntimeError> {
    let state = state.lock().await;
    state.sessions.update(session.id, |s| {
        s.char = Some(char.clone());
    });
    Ok(())
}
