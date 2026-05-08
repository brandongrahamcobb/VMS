use crate::config::settings;
use crate::inc::helpers;
use crate::models::account::model::AccountModel;
use crate::models::channel::model::ChannelModel;
use crate::models::character::model::CharacterModel;
use crate::models::map::model::MapModel;
use crate::models::world::model::WorldModel;
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
                            acc: AccountModel::new(),
                            authenticated: false,
                            playing: false,
                            hwid: String::new(),
                            world: WorldModel::new(),
                            channel: ChannelModel::new(),
                            map: MapModel::new(),
                            char: CharacterModel::new(),
                            tx: tx.clone(),
                        })
                    };
                    info!("Listening on port {}...", port);
                    let state = state.clone();
                    tokio::spawn(async move {
                        match Runtime::<LoginRelay>::new(state, stream, session_id, rx).await {
                            Ok(mut runtime) => match runtime.run().await {
                                Ok(Some(id)) => {
                                    let port = {
                                        let state = state.lock().await;
                                        match state.sessions.get(id) {
                                            Some(s) => s.channel.port,
                                            None => {
                                                info!(
                                                    "Expected a valid session. Session ID: {}",
                                                    id
                                                );
                                                return;
                                            }
                                        }
                                    };
                                    tokio::spawn(PlayerServer::accept(state, id, port));
                                }
                                Ok(None) => {
                                    let state = state.lock().await;
                                    state.sessions.remove(session_id);
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
    pub fn accept(
        state: SharedState,
        session_id: i32,
        port: u16,
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
                    tokio::spawn(async move {
                        match Runtime::<PlayerRelay>::new(state.clone(), stream, session_id, rx)
                            .await
                        {
                            Ok(mut runtime) => match runtime.run().await {
                                Ok(Some(id)) => {
                                    let port = {
                                        let state = state.lock().await;
                                        match state.sessions.get(id) {
                                            Some(s) => s.channel.port,
                                            None => {
                                                info!(
                                                    "Expected a valid session. Session ID: {}",
                                                    id
                                                );
                                                return;
                                            }
                                        }
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
