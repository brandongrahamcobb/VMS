use crate::net::packet::model::Packet;
use crate::runtime::error::RuntimeError;
use crate::runtime::relay::scope::{ChannelScope, MapScope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub async fn simply_send_locally(
    _state: &SharedState,
    session: &Session,
    packet: &Packet,
) -> Result<(), RuntimeError> {
    session.tx.send(packet.clone())?;
    Ok(())
}

pub async fn simply_send_to_map(
    state: &SharedState,
    session: &Session,
    packet: &Packet,
    map_scope: &MapScope,
) -> Result<(), RuntimeError> {
    match map_scope {
        MapScope::SameChannelSameWorld => {
            let state = state.lock().await;
            let sessions = state.sessions.get_by_map_channel_world(
                session.get_map()?.model.wz_id,
                session.get_channel()?.model.id,
                session.get_world()?.model.id,
                session.id,
            );
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
        MapScope::AllChannelsSameWorld => {
            let state = state.lock().await;
            let sessions = state.sessions.get_by_map_world(
                session.get_map()?.model.wz_id,
                session.get_world()?.model.id,
                session.id,
            );
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
        MapScope::AllChannelsAllWorlds => {
            let state = state.lock().await;
            let sessions = state
                .sessions
                .get_by_map(session.get_map()?.model.wz_id, session.id);
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
    packet: &Packet,
    channel_scope: &ChannelScope,
) -> Result<(), RuntimeError> {
    match channel_scope {
        ChannelScope::SameWorld => {
            let state = state.lock().await;
            let sessions = state.sessions.get_by_channel_world(
                session.get_channel()?.model.id,
                session.get_world()?.model.id,
                session.id,
            );
            for s in sessions {
                s.tx.send(packet.clone())?;
            }
        }
        ChannelScope::AllWorlds => {
            let state = state.lock().await;
            let sessions = state
                .sessions
                .get_by_channel(session.get_channel()?.model.id, session.id);
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
    packet: &Packet,
) -> Result<(), RuntimeError> {
    let state = state.lock().await;
    let sessions = state
        .sessions
        .get_by_world(session.get_world()?.model.id, session.id);
    for s in sessions {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}

pub async fn simply_send_globally(
    state: &SharedState,
    session: &Session,
    packet: &Packet,
) -> Result<(), RuntimeError> {
    let state = state.lock().await;
    let sessions = state.sessions.get_all(session.id);
    for s in sessions {
        s.tx.send(packet.clone())?;
    }
    Ok(())
}
