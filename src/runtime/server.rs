use crate::config::settings;
use crate::runtime::error::{RuntimeError, SessionError};
use crate::runtime::relay::{LoginRelay, PlayerRelay, Runtime};
use crate::runtime::state::SharedState;
use tracing::info;

pub struct LoginServer;

impl LoginServer {
    pub async fn run(state: SharedState) -> Result<(), RuntimeError> {
        let port = settings::get_login_port()?;
        let addr = settings::build_server_addr(&port)?;
        let listener = tokio::net::TcpListener::bind(addr.clone()).await?;
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                    let session_id = {
                        let state = state.lock().await;
                        state.sessions.insert(Session {
                            id: 0,
                            acc: None,
                            authenticated: false,
                            playing: false,
                            hwid: None,
                            world: None,
                            channel: None,
                            map: None,
                            char: None,
                            tx: tx.clone(),
                        })
                    };
                    info!("Listening on port {}...", &port);
                    let state = state.clone();
                    tokio::spawn(async move {
                        match Runtime::<LoginRelay>::new(state, stream, session_id, rx).await {
                            Ok(mut runtime) => match runtime.run().await {
                                Ok(Some(id)) => {
                                    let port = {
                                        let state = state.lock().await;
                                        match state.sessions.get(&id) {
                                            Some(s) => s.channel.as_ref().map(|c| c.port),
                                            None => {
                                                info!(
                                                    "Expected a valid session. Session ID: {}",
                                                    id
                                                );
                                                return;
                                            }
                                        }
                                    };
                                    match port {
                                        Some(port) => {
                                            tokio::spawn(PlayerServer::accept(state, id, port));
                                        }
                                        None => info!(
                                            "Expected a channel in session. Session ID: {}",
                                            id
                                        ),
                                    }
                                }
                                Ok(None) => {
                                    let state = state.lock().await;
                                    state.sessions.remove(&session_id);
                                }
                                Err(e) => info!("Expected a session ID. Error: {}", e.to_string()),
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
    pub async fn accept(state: SharedState, session_id: i32, port: i16) {
        let addr = match settings::build_server_addr(&port) {
            Ok(a) => a,
            Err(e) => {
                info!("Expected valid player addr. Error: {}", e);
                return;
            }
        };
        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(l) => l,
            Err(e) => {
                info!("Expected valid player listener. Error: {}", e);
                return;
            }
        };
        match listener.accept().await {
            Ok((stream, _)) => {
                let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                {
                    let state = state.lock().await;
                    state.sessions.update(&session_id, |s| s.tx = tx.clone());
                }
                info!("Listening on port {}...", &port);
                tokio::spawn(async move {
                    match Runtime::<PlayerRelay>::new(state.clone(), stream, session_id, rx).await {
                        Ok(mut runtime) => match runtime.run().await {
                            Ok(Some(id)) => {
                                let port = {
                                    let state = state.lock().await;
                                    match state.sessions.get(&id) {
                                        Some(s) => s.channel.as_ref().map(|c| c.port),
                                        None => {
                                            info!("Expected a valid session. Session ID: {}", id);
                                            return;
                                        }
                                    }
                                };
                                match port {
                                    Some(port) => {
                                        tokio::spawn(PlayerServer::accept(state.clone(), id, port));
                                    }
                                    None => {
                                        info!("Expected a channel in session. Session ID: {}", id)
                                    }
                                }
                            }
                            Ok(None) => {
                                let state = state.lock().await;
                                state.sessions.remove(&session_id);
                            }
                            Err(e) => info!("Expected a session ID. Error: {}", e.to_string()),
                        },
                        Err(e) => info!("Expected a player runtime. Error: {}", e.to_string()),
                    };
                });
            }
            Err(e) => info!("Expected valid player listener. Error: {}", e.to_string()),
        }
    }
}
