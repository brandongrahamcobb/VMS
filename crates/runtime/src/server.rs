/* server.rs
 * The purpose of this module is to provide the connection to the client.
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

use crate::error::RuntimeError;
use crate::relay::model::{LoginRelay, PlayerRelay, Runtime};
use config::settings;
use core::pin::Pin;
use inc::helpers;
use session::model::Session;
use state::model::SharedState;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct LoginServer;

impl LoginServer {
    pub async fn run(state: SharedState) -> Result<(), RuntimeError> {
        let port = settings::get_login_port()?;
        let addr = settings::get_bind_address()?;
        let bind = helpers::build_server_addr(addr, port);
        let listener: TcpListener = TcpListener::bind(bind).await?;
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                    let session_id = {
                        let state = state.lock().await;
                        state.sessions.insert(Session {
                            id: 0,
                            acc_id: None,
                            channel_id: None,
                            char_id: None,
                            map_wz: None,
                            world_id: None,
                            tx: tx.clone(),
                            transitioning: true,
                        })
                    };
                    info!("Listening on port {}...", port);
                    let state = state.clone();
                    tokio::spawn(async move {
                        match Runtime::<LoginRelay>::new(state.clone(), stream, session_id, rx)
                            .await
                        {
                            Ok(runtime) => match runtime.run().await {
                                Ok(Some((mut runtime, mut packet))) => {
                                    let id = runtime.relay.session_id;
                                    let port = {
                                        let state = state.lock().await;
                                        let Some(session) = state.sessions.get(id) else {
                                            info!(
                                                "Expected a valid login session for transitioning to player server. Session ID: {}",
                                                id
                                            );
                                            return;
                                        };
                                        let Ok(world_id) = session.get_world_id() else {
                                            info!(
                                                "Expected a login session world ID for transitioning to player server. Session ID: {}",
                                                id
                                            );
                                            return;
                                        };
                                        let Ok(channel_id) = session.get_channel_id() else {
                                            info!(
                                                "Expected a login session channel ID for transitioning to player server. Session ID: {}",
                                                id
                                            );
                                            return;
                                        };
                                        match state
                                            .with_channel(world_id, channel_id, |channel| {
                                                channel.model.port
                                            })
                                            .await
                                        {
                                            Ok(port) => port,
                                            Err(_) => {
                                                info!(
                                                    "Expected a game state world and channel for transitioning to player server. Session ID: {}",
                                                    id
                                                );
                                                return;
                                            }
                                        }
                                    };
                                    tokio::spawn(accept(state, id, port));
                                    tokio::task::yield_now().await;
                                    match runtime
                                        .pkt_writer
                                        .send_encrypted_packet(&mut packet)
                                        .await
                                    {
                                        Ok(_) => {}
                                        Err(e) => info!(
                                            "Expected to send a final packet. Error: {}",
                                            e.to_string()
                                        ),
                                    }
                                }
                                Ok(None) => {
                                    let state = state.lock().await;
                                    state.sessions.remove(session_id);
                                }
                                Err(e) => {
                                    use std::error::Error;
                                    let mut current: Option<&dyn Error> = Some(&e);
                                    while let Some(err) = current {
                                        println!("{}", err);
                                        current = err.source();
                                    }
                                    let state = state.lock().await;
                                    state.sessions.remove(session_id);
                                    info!("Expected a session ID. Error: {}", e.to_string());
                                }
                            },
                            Err(e) => info!("Expected a login runtime. Error: {}", e.to_string(),),
                        };
                    });
                }
                Err(e) => info!("Expected valid login listener. Error: {}", e,),
            }
        }
    }
}

pub struct PlayerServer;

impl PlayerServer {
    pub async fn run(state: SharedState) -> Result<(), RuntimeError> {
        let addr = settings::get_bind_address()?;
        let ports = {
            let mut ports: Vec<i16> = Vec::new();
            let state = state.lock().await;
            let worlds = state.worlds.read().await;
            for (_, world) in worlds.iter() {
                for (_, channel) in world.channels.iter() {
                    ports.push(channel.model.port);
                }
            }
            ports
        };
        for port in ports {
            let bind = helpers::build_server_addr(addr.clone(), port);
            let listener: TcpListener = TcpListener::bind(bind).await?;
            let mut state = state.lock().await;
            state.listeners.insert(port, Arc::new(listener));
        }
        Ok(())
    }
}

pub fn accept(
    state: SharedState,
    session_id: i32,
    port: i16,
) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        let listener = state.lock().await.listeners.get(&port).unwrap().clone();
        match listener.accept().await {
            Ok((stream, _)) => {
                let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                {
                    let state = state.lock().await;
                    state.sessions.update(session_id, |s| s.tx = tx.clone());
                }
                info!("Listening on port {}...", port);
                let state = state.clone();
                tokio::spawn(async move {
                    match Runtime::<PlayerRelay>::new(state.clone(), stream, session_id, rx).await {
                        Ok(runtime) => match runtime.run().await {
                            Ok(Some((mut runtime, mut packet))) => {
                                let id = runtime.relay.session_id;
                                let port = {
                                    let state = state.lock().await;
                                    let Some(session) = state.sessions.get(id) else {
                                        info!("Expected a valid session. Session ID: {}", id);
                                        return;
                                    };
                                    let Ok(world_id) = session.get_world_id() else {
                                        info!(
                                            "Expected a player session world ID for transitioning between player servers. Session ID: {}",
                                            id
                                        );
                                        return;
                                    };
                                    let Ok(channel_id) = session.get_channel_id() else {
                                        info!(
                                            "Expected a player session channel ID for transitioning between player servers. Session ID: {}",
                                            id
                                        );
                                        return;
                                    };
                                    match state
                                        .with_channel(world_id, channel_id, |channel| {
                                            channel.model.port
                                        })
                                        .await
                                    {
                                        Ok(port) => port,
                                        Err(_) => {
                                            info!(
                                                "Expected a game state world and channel for transitioning to player server. Session ID: {}",
                                                id
                                            );
                                            return;
                                        }
                                    }
                                };
                                tokio::spawn(accept(state.clone(), id, port));
                                tokio::task::yield_now().await;
                                match runtime.pkt_writer.send_encrypted_packet(&mut packet).await {
                                    Ok(_) => {}
                                    Err(e) => info!(
                                        "Expected to send a final packet. Error: {}",
                                        e.to_string()
                                    ),
                                }
                            }
                            Ok(None) => {
                                let state = state.lock().await;
                                state.sessions.remove(session_id);
                            }
                            Err(e) => {
                                use std::error::Error;
                                let mut current: Option<&dyn Error> = Some(&e);
                                while let Some(err) = current {
                                    println!("{}", err);
                                    current = err.source();
                                }
                                let state = state.lock().await;
                                state.sessions.remove(session_id);
                                info!("Expected a session ID. Error: {}", e.to_string());
                            }
                        },
                        Err(e) => {
                            info!("Expected a player runtime. Error: {}", e.to_string())
                        }
                    };
                });
            }
            Err(e) => info!("Expected valid player listener. Error: {}", e.to_string()),
        }
    })
}
