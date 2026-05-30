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
use core::net::SocketAddr;
use core::pin::Pin;
use inc::helpers;
use ipc::channel::{TcpCommand, TcpEvent};
use session::model::Session;
use state::model::SharedState;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use tokio::net::TcpListener;
use tokio::try_join;
use tracing::info;

pub struct LoginServer;

impl LoginServer {
    pub async fn run(
        command_rx: Arc<Mutex<Receiver<TcpCommand>>>,
        event_tx: Sender<TcpEvent>,
    ) -> Result<(), RuntimeError> {
        let port = settings::get_login_port()?;
        let addr = settings::get_bind_address()?;
        let bind = helpers::build_server_addr(addr, port);
        let listener: TcpListener = TcpListener::bind(bind).await?;
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let client_id = ipc::client::next_client_id();
                    let command_rx = Arc::clone(&command_rx);
                    let event_tx = event_tx.clone();
                    event_tx
                        .send(TcpEvent::ClientConnected { client_id })
                        .unwrap();
                    tokio::spawn(async move {
                        match Runtime::<LoginRelay>::new(stream, client_id).await {
                            Ok(runtime) => match runtime.run(command_rx, event_tx).await {
                                Ok(_) => {}
                                Err(e) => {
                                    info!("Login server error: {}", e);
                                    event_tx
                                        .send(TcpEvent::ClientDisconnected { client_id })
                                        .unwrap();
                                }
                            },
                            Err(e) => info!("Login server init error: {}", e),
                        };
                    });
                }
                Err(e) => info!("Login listener error: {}", e),
            }
        }
    }
}

pub struct PlayerServer;

impl PlayerServer {
    pub async fn run(
        command_rx: Arc<Mutex<Receiver<TcpCommand>>>,
        event_tx: Sender<TcpEvent>,
    ) -> Result<(), RuntimeError> {
        let addr = settings::get_bind_address()?;
        loop {
            let cmd = { command_rx.lock().unwrap().try_recv() };
            if let Ok(TcpCommand::AcceptTransition { client_id, port }) = cmd {
                let command_rx = Arc::clone(&command_rx);
                let event_tx = event_tx.clone();
                let bind: SocketAddr = helpers::build_server_addr(addr, port);
                let listener = TcpListener::bind(bind).await?;
                tokio::spawn(async move {
                    match listener.accept().await {
                        Ok((stream, _)) => {
                            event_tx
                                .send(TcpEvent::ClientConencted { client_id })
                                .unwrap();
                            match Runtime::<PlayerRelay>::new(stream, client_id).await {
                                Ok(runtime) => match runtime.run(command_rx, event_tx).await {
                                    Ok(_) => {}
                                    Err(e) => info!("Player runtime error: {}", e),
                                },
                                Err(e) => {
                                    event_tx
                                        .send(TcpEvent::ClientDisconnected { client_id })
                                        .unwrap();
                                    info!("Player runtime init error: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            info!("Player listener error: {}", e)
                        }
                    };
                });
            }
            tokio::task::yield_now().await;
        }
    }
}
