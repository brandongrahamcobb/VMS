/* session_execute.rs
 * The purpose of this module is to provide action-wide functions for session-related actions.
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
use crate::relay::execute::{set_channel, set_map, set_world, simple};
use net::action::scope::SessionScope;
use core::ops::ControlFlow;
use entity::map::constants::VACANCY_DURATION;
use entity::map::model::{MapModel, Point, VacancyState};
use net::packet::handler::result::HandlerResult;
use packet::model::Packet;
use session::model::Session;
use state::model::SharedState;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

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
    {
        let state = state.lock().await;
        state
            .with_mut_map(
                world_id,
                channel_id,
                map_wz,
                |map| -> Result<(), ExecuteError> {
                    if let Some(token) = map.vacancy_token.take() {
                        token.cancel();
                    }
                    let mut packets: Vec<Packet> = Vec::<Packet>::new();
                    for player in map.chars.values() {
                        packets.push(
                            Packet::new_empty()
                                .build_spawn_player_packet(player)?
                                .finish(),
                        );
                    }
                    for (mob_id, mob) in map.mobs.iter() {
                        packets.push(
                            Packet::new_empty()
                                .build_spawn_mob_packet(*mob_id, &mob.life)?
                                .finish(),
                        );
                        packets.push(
                            Packet::new_empty()
                                .build_spawn_mob_controller_packet(
                                    *mob_id,
                                    1,
                                    mob.life.wz as i32,
                                    0,
                                    mob.model.fh,
                                    0,
                                    &Point {
                                        x: mob.model.pos_x,
                                        y: mob.model.pos_y,
                                    },
                                    -1,
                                )?
                                .finish(),
                        );
                    }
                    for packet in packets {
                        session.tx.send(packet)?;
                    }
                    Ok(())
                },
            )
            .await??
    };
    Ok(())
}

pub async fn enter_map(
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

pub async fn exit_map(state: &SharedState, session: &Session) -> Result<(), ExecuteError> {
    let world_id: i16 = session.get_world_id()?;
    let channel_id: u8 = session.get_channel_id()?;
    let map_wz: i32 = session.get_map_wz()?;
    let char_id: i32 = session.get_char_id()?;
    let is_vacant = {
        let state = state.lock().await;
        state
            .with_mut_map(
                world_id,
                channel_id,
                map_wz,
                |map| -> Result<bool, ExecuteError> {
                    map.chars.remove(&char_id);
                    Ok(map.chars.is_empty())
                },
            )
            .await??
    };
    if is_vacant {
        let token: CancellationToken = CancellationToken::new();
        let token_clone: CancellationToken = token.clone();
        let state_clone = Arc::clone(state);
        {
            let state = state.lock().await;
            state
                .with_mut_map(world_id, channel_id, map_wz, |map| {
                    map.vacancy = VacancyState::Vacant;
                    map.vacancy_token = Some(token);
                })
                .await?;
        }
        tokio::spawn(async move {
            tokio::select! {
                _ = tokio::time::sleep(VACANCY_DURATION) => {
                    let _ = suspend_map(&state_clone, world_id, channel_id, map_wz).await;
                }
                _ = token_clone.cancelled() => {}
            }
        });
    }
    Ok(())
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
    let exists = {
        let state = state.lock().await;
        state
            .with_channel(world_id, channel_id, |channel| {
                channel.maps.contains_key(&map_wz)
            })
            .await?
    };
    if !exists {
        let map_model: MapModel = MapModel { wz: map_wz };
        let map = assembly::map::assemble::assemble_map_by_map_wz(map_wz)?;
        let state = state.lock().await;
        state
            .with_mut_channel(world_id, channel_id, |channel| {
                channel.maps.insert(map_wz, map);
            })
            .await?;
    }
    let state = state.lock().await;
    let tick_rx = state
        .with_channel(world_id, channel_id, |channel| {
            channel.maps.get(&map_wz).unwrap().tick_tx.subscribe()
        })
        .await?;
    Ok(tick_rx)
}

pub async fn suspend_map(
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
