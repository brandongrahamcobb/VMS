use crate::models::shroom::map::model::Map;
use crate::runtime::error::RuntimeError;
use crate::runtime::relay::scope::{ChannelScope, MapScope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub async fn set_map_locally(
    state: &SharedState,
    session: &Session,
    map: &Map,
) -> Result<(), RuntimeError> {
    let state = state.lock().await;
    state.sessions.update(session.id, |s| {
        s.map = Some(map.clone());
    });
    Ok(())
}

pub async fn set_map_for_map(
    state: &SharedState,
    session: &Session,
    map: &Map,
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
                state.sessions.update(s.id, |s| {
                    s.map = Some(map.clone());
                });
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
                state.sessions.update(s.id, |s| {
                    s.map = Some(map.clone());
                });
            }
        }
        MapScope::AllChannelsAllWorlds => {
            let state = state.lock().await;
            let sessions = state
                .sessions
                .get_by_map(session.get_map()?.model.wz_id, session.id);
            for s in sessions {
                state.sessions.update(s.id, |s| {
                    s.map = Some(map.clone());
                });
            }
        }
    }
    Ok(())
}

pub async fn set_map_for_channel(
    state: &SharedState,
    session: &Session,
    map: &Map,
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
                state.sessions.update(s.id, |s| {
                    s.map = Some(map.clone());
                });
            }
        }
        ChannelScope::AllWorlds => {
            let state = state.lock().await;
            let sessions = state
                .sessions
                .get_by_channel(session.get_channel()?.model.id, session.id);
            for s in sessions {
                state.sessions.update(s.id, |s| {
                    s.map = Some(map.clone());
                });
            }
        }
    }
    Ok(())
}

pub async fn set_map_for_world(
    state: &SharedState,
    session: &Session,
    map: &Map,
) -> Result<(), RuntimeError> {
    let state = state.lock().await;
    let sessions = state
        .sessions
        .get_by_world(session.get_world()?.model.id, session.id);
    for s in sessions {
        state.sessions.update(s.id, |s| {
            s.map = Some(map.clone());
        });
    }
    Ok(())
}

pub async fn set_map_globally(
    state: &SharedState,
    session: &Session,
    map: &Map,
) -> Result<(), RuntimeError> {
    let state = state.lock().await;
    let sessions = state.sessions.get_all(session.id);
    for s in sessions {
        state.sessions.update(s.id, |s| {
            s.map = Some(map.clone());
        });
    }
    Ok(())
}
