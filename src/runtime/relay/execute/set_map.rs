/* set_map.rs
 * The purpose of this module is to provide the set map functions for relay handling.
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

use crate::db::error::DatabaseError;
use crate::models::character;
use crate::models::character::wrapper::Character;
use crate::runtime::relay::execute::error::ExecuteError;
use crate::runtime::relay::scope::{ChannelScope, MapScope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub async fn set_map_locally(
    state: &SharedState,
    session: &Session,
    map_wz: i32,
) -> Result<(), ExecuteError> {
    let char_id: i32 = session.get_char_id()?;
    let mut char: Character = character::service::get_char_by_id(state, char_id).await?;
    char.model.map_wz = map_wz;
    character::query::setters::update_characters(state, vec![char.model])
        .await
        .map_err(|e| DatabaseError::DieselError(e))?;
    let locked_state = state.lock().await;
    locked_state.sessions.update(session.id, |s| {
        s.map_wz = Some(map_wz);
    });
    Ok(())
}

pub async fn set_map_for_map(
    state: &SharedState,
    session: &Session,
    map_scope: &MapScope,
    map_wz: i32,
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
                    s.map_wz = Some(map_wz);
                });
                let char_id: i32 = s.get_char_id()?;
                let mut char: Character =
                    character::service::get_char_by_id(state, char_id).await?;
                char.model.map_wz = map_wz;
                character::query::setters::update_characters(state, vec![char.model])
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
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
                    s.map_wz = Some(map_wz);
                });
                let char_id: i32 = s.get_char_id()?;
                let mut char: Character =
                    character::service::get_char_by_id(state, char_id).await?;
                char.model.map_wz = map_wz;
                character::query::setters::update_characters(state, vec![char.model])
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
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
                    s.map_wz = Some(map_wz);
                });
                let char_id: i32 = s.get_char_id()?;
                let mut char: Character =
                    character::service::get_char_by_id(state, char_id).await?;
                char.model.map_wz = map_wz;
                character::query::setters::update_characters(state, vec![char.model])
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
            }
        }
    }
    Ok(())
}

pub async fn set_map_for_channel(
    state: &SharedState,
    session: &Session,
    channel_scope: &ChannelScope,
    map_wz: i32,
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
                    s.map_wz = Some(map_wz);
                });
                let char_id: i32 = s.get_char_id()?;
                let mut char: Character =
                    character::service::get_char_by_id(state, char_id).await?;
                char.model.map_wz = map_wz;
                character::query::setters::update_characters(state, vec![char.model])
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
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
                    s.map_wz = Some(map_wz);
                });
                let char_id: i32 = s.get_char_id()?;
                let mut char: Character =
                    character::service::get_char_by_id(state, char_id).await?;
                char.model.map_wz = map_wz;
                character::query::setters::update_characters(state, vec![char.model])
                    .await
                    .map_err(|e| DatabaseError::DieselError(e))?;
            }
        }
    }
    Ok(())
}

pub async fn set_map_for_world(
    state: &SharedState,
    session: &Session,
    map_wz: i32,
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
            s.map_wz = Some(map_wz);
        });
        let char_id: i32 = s.get_char_id()?;
        let mut char: Character = character::service::get_char_by_id(state, char_id).await?;
        char.model.map_wz = map_wz;
        character::query::setters::update_characters(state, vec![char.model])
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
    }
    Ok(())
}

pub async fn set_map_globally(
    state: &SharedState,
    session: &Session,
    map_wz: i32,
) -> Result<(), ExecuteError> {
    let sessions = {
        let locked_state = state.lock().await;
        locked_state.sessions.get_all(session.id)
    };
    for s in sessions {
        let locked_state = state.lock().await;
        locked_state.sessions.update(s.id, |s| {
            s.map_wz = Some(map_wz);
        });
        let char_id: i32 = s.get_char_id()?;
        let mut char: Character = character::service::get_char_by_id(state, char_id).await?;
        char.model.map_wz = map_wz;
        character::query::setters::update_characters(state, vec![char.model])
            .await
            .map_err(|e| DatabaseError::DieselError(e))?;
    }
    Ok(())
}
