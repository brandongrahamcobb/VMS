use crate::config::settings;
use crate::inc::helpers;
use crate::runtime::error::RuntimeError;
use crate::runtime::relay::{LoginRelay, PlayerRelay, Runtime};
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::future::Future;
use std::pin::Pin;
use tokio::net::TcpListener;
use tracing::info;

pub struct LoginServer;

impl LoginServer {
    pub async fn run(state: SharedState) -> Result<(), RuntimeError> {
        let port = settings::get_login_port()?;
        let addr = settings::get_address()?;
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
                            acc: None,
                            hwid: None,
                            world: None,
                            channel: None,
                            map: None,
                            char: None,
                            tx: tx.clone(),
                            playing: true,
                        })
                    };
                    info!("Listening on port {}...", port);
                    let state = state.clone();
                    tokio::spawn(async move {
                        match Runtime::<LoginRelay>::new(state.clone(), stream, session_id, rx)
                            .await
                        {
                            Ok(mut runtime) => match runtime.run().await {
                                Ok(Some(id)) => {
                                    let port = {
                                        let state = state.lock().await;
                                        let Some(session) = state.sessions.get(id) else {
                                            info!("Expected a valid session. Session ID: {}", id);
                                            return;
                                        };
                                        let Some(channel) = session.channel else {
                                            info!("Expected a valid channel. Session ID: {}", id);
                                            return;
                                        };
                                        channel.model.port
                                    };
                                    tokio::spawn(PlayerServer::accept(state, id, port));
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
    pub fn accept(
        state: SharedState,
        session_id: i32,
        port: i16,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            let addr = match settings::get_address() {
                Ok(a) => a,
                Err(e) => {
                    info!("Expected valid player addr. Error: {}", e);
                    return;
                }
            };
            let bind = helpers::build_server_addr(addr, port);
            let listener: TcpListener = match TcpListener::bind(bind).await {
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
                        state.sessions.update(session_id, |s| s.tx = tx.clone());
                    }
                    info!("Listening on port {}...", port);
                    let state = state.clone();
                    tokio::spawn(async move {
                        match Runtime::<PlayerRelay>::new(state.clone(), stream, session_id, rx)
                            .await
                        {
                            Ok(mut runtime) => match runtime.run().await {
                                Ok(Some(id)) => {
                                    let port = {
                                        let state = state.lock().await;
                                        let Some(session) = state.sessions.get(id) else {
                                            info!("Expected a valid session. Session ID: {}", id);
                                            return;
                                        };
                                        let Some(channel) = session.channel else {
                                            info!("Expected a valid channel. Session ID: {}", id);
                                            return;
                                        };
                                        channel.model.port
                                    };
                                    tokio::spawn(PlayerServer::accept(state.clone(), id, port));
                                }
                                Ok(None) => {
                                    let state = state.lock().await;
                                    state.sessions.remove(session_id);
                                }
                                Err(e) => info!("Expected a session ID. Error: {}", e.to_string()),
                            },
                            Err(e) => info!("Expected a player runtime. Error: {}", e.to_string()),
                        };
                    });
                }
                Err(e) => info!("Expected valid player listener. Error: {}", e.to_string()),
            }
        })
    }
}
