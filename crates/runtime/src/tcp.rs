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
use db::pool::DbPool;
use inc::helpers;
use ipc::channel::{AsyncCommand, AsyncEvent};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::try_join;
use tracing::info;

pub async fn start_server(
    event_tx: Sender<AsyncEvent>,
    command_rx: Receiver<AsyncCommand>,
    db: DbPool,
) -> Result<(), RuntimeError> {
    let command_rx = Arc::new(Mutex::new(command_rx));
    // tokio::spawn({
    //     let command_tx = Arc::clone(&command_rx);
    //     let db = db.clone();
    //     let event_tx = event_tx.clone();
    //     async move {
    //         login_worker(command_rx, db, event_tx).await;
    //     }
    // });

    info!("Binding to login server...");
    let login = LoginServer::run(event_tx.clone(), Arc::clone(&command_rx));

    info!("Binding to player server...");
    let player = PlayerServer::run(event_tx.clone(), Arc::clone(&command_rx));

    let (_, _) = try_join!(login, player)?;
    Ok(())
}

pub struct LoginServer;

impl LoginServer {
    pub async fn run(
        command_rx: Arc<Mutex<Receiver<AsyncCommand>>>,
        event_tx: Sender<AsyncEvent>,
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
                        .send(AsyncEvent::ClientConnected { client_id })
                        .unwrap();
                    tokio::spawn(async move {
                        match Runtime::<LoginRelay>::new(stream, client_id).await {
                            Ok(runtime) => match runtime.run(command_rx, event_tx).await {
                                Ok(_) => {}
                                Err(e) => {
                                    info!("Login server error: {}", e);
                                    event_tx
                                        .send(AsyncEvent::ClientDisconnected { client_id })
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
        command_rx: Arc<Mutex<Receiver<AsyncCommand>>>,
        event_tx: Sender<AsyncEvent>,
    ) -> Result<(), RuntimeError> {
        let addr = settings::get_bind_address()?;
        loop {
            let cmd = { command_rx.lock().unwrap().try_recv() };
            if let Ok(AsyncCommand::AcceptTransition { client_id, port }) = cmd {
                let command_rx = Arc::clone(&command_rx);
                let event_tx = event_tx.clone();
                let bind: SocketAddr = helpers::build_server_addr(addr, port);
                let listener = TcpListener::bind(bind).await?;
                tokio::spawn(async move {
                    match listener.accept().await {
                        Ok((stream, _)) => {
                            event_tx
                                .send(AsyncEvent::ClientConencted { client_id })
                                .unwrap();
                            match Runtime::<PlayerRelay>::new(stream, client_id).await {
                                Ok(runtime) => match runtime.run(command_rx, event_tx).await {
                                    Ok(_) => {}
                                    Err(e) => info!("Player runtime error: {}", e),
                                },
                                Err(e) => {
                                    event_tx
                                        .send(AsyncEvent::ClientDisconnected { client_id })
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
