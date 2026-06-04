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
use crate::relay::Runtime;
use crate::worker::db_worker;
use config::settings;
use core::net::SocketAddr;
use db::pool::DbPool;
use inc::helpers;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;
use ipc::asyncronous::event::AsyncEvent;
use net::packet::model::Packet;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tracing::info;

pub struct Register {
    pub id: i32,
    pub tx: mpsc::Sender<Packet>,
}

pub async fn start_server(
    command_rx: Receiver<AsyncCommand>,
    event_tx: Sender<AsyncEvent>,
    pool: DbPool,
) -> Result<(), RuntimeError> {
    let mut relays: HashMap<i32, mpsc::Sender<Packet>> = HashMap::new();
    let (register_tx, mut register_rx) = mpsc::channel::<Register>(32);
    let (transition_tx, transition_rx) = mpsc::channel::<AsyncCommand>(32);
    let (db_tx, db_rx) = mpsc::channel::<DatabaseCommand>(32);

    tokio::spawn(LoginServer::run(event_tx.clone(), register_tx.clone()));
    tokio::spawn(PlayerServer::run(
        event_tx.clone(),
        register_tx.clone(),
        transition_rx,
    ));
    tokio::spawn(db_worker(db_rx, pool, event_tx.clone()));

    loop {
        while let Ok(client) = register_rx.try_recv() {
            relays.insert(client.id, client.tx);
        }
        while let Ok(cmd) = command_rx.try_recv() {
            dbg!("command received");
            match cmd {
                AsyncCommand::SendPacket { client_id, packet } => {
                    if let Some(tx) = relays.get(&client_id) {
                        let _ = tx.send(packet);
                    }
                }
                AsyncCommand::AcceptTransition { .. } => {
                    transition_tx.send(cmd).await?;
                }
                AsyncCommand::DatabaseOperation(cmd) => {
                    db_tx.send(cmd).await?;
                }
                _ => {}
            }
        }
        tokio::task::yield_now().await;
    }
}

pub struct LoginServer;

impl LoginServer {
    pub async fn run(
        event_tx: Sender<AsyncEvent>,
        register_tx: mpsc::Sender<Register>,
    ) -> Result<(), RuntimeError> {
        let port = settings::get_login_port()?;
        let addr = settings::get_bind_address()?;
        let bind = helpers::build_server_addr(addr, port);
        let listener: TcpListener = TcpListener::bind(bind).await?;
        loop {
            let event_tx = event_tx.clone();
            match listener.accept().await {
                Ok((stream, _)) => {
                    let client_id = crate::client::next_client_id();
                    let (tx, rx) = mpsc::channel::<Packet>(32);
                    register_tx.send(Register { id: client_id, tx }).await?;
                    event_tx
                        .send(AsyncEvent::ClientConnected { client_id })
                        .unwrap();
                    tokio::spawn(async move {
                        match Runtime::new(stream, None).await {
                            Ok(runtime) => match runtime.run(client_id, event_tx.clone(), rx).await
                            {
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
            tokio::task::yield_now().await;
        }
    }
}

pub struct PlayerServer;

impl PlayerServer {
    pub async fn run(
        event_tx: Sender<AsyncEvent>,
        register_tx: mpsc::Sender<Register>,
        mut transition_rx: mpsc::Receiver<AsyncCommand>,
    ) -> Result<(), RuntimeError> {
        let addr = settings::get_bind_address()?;
        loop {
            if let Some(AsyncCommand::AcceptTransition {
                client_id,
                port,
                packet,
            }) = transition_rx.recv().await
            {
                let bind: SocketAddr = helpers::build_server_addr(addr.clone(), port);
                let listener = TcpListener::bind(bind).await?;
                let event_tx = event_tx.clone();
                let register_tx = register_tx.clone();
                match listener.accept().await {
                    Ok((stream, _)) => {
                        let (tx, rx) = mpsc::channel::<Packet>(32);
                        register_tx.send(Register { id: client_id, tx }).await?;
                        event_tx
                            .send(AsyncEvent::ClientConnected { client_id })
                            .unwrap();
                        match Runtime::new(stream, Some(packet)).await {
                            Ok(runtime) => {
                                match runtime.run(client_id, event_tx.clone(), rx).await {
                                    Ok(_) => {}
                                    Err(e) => info!("Player runtime error: {}", e),
                                }
                            }
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
            }
            tokio::task::yield_now().await;
        }
    }
}
