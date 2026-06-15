/* runtime/src/tcp.rs
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

use crate::ddos_protection::{self, ConnectedIps, IpGuard, RateLimiter};
use crate::error::RuntimeError;
use crate::relay::Runtime;
use crate::worker::db_worker;
use config::settings;
use core::net::SocketAddr;
use core::time::Duration;
use db::pool::DbPool;
use inc::helpers;
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;
use ipc::event::AsyncEvent;
use net::packet::model::Packet;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::{Semaphore, mpsc};
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
    let limiter: RateLimiter = Arc::new(Mutex::new(HashMap::new()));
    let connection_semaphore = Arc::new(tokio::sync::Semaphore::new(2000));

    let mut relays: HashMap<i32, mpsc::Sender<Packet>> = HashMap::new();
    let (register_tx, mut register_rx) = mpsc::channel::<Register>(32);
    let (transition_tx, transition_rx) = mpsc::channel::<AsyncCommand>(32);
    let (db_tx, db_rx) = mpsc::channel::<DatabaseCommand>(32);
    let connected_ips: ConnectedIps = Arc::new(Mutex::new(HashSet::new()));

    {
        let limiter = limiter.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
                ddos_protection::sweep(&limiter);
            }
        });
    }
    tokio::spawn(LoginServer::run(
        event_tx.clone(),
        register_tx.clone(),
        connected_ips.clone(),
        limiter.clone(),
        connection_semaphore.clone(),
    ));
    tokio::spawn(PlayerServer::run(
        event_tx.clone(),
        register_tx.clone(),
        transition_rx,
        connected_ips.clone(),
        limiter.clone(),
        connection_semaphore.clone(),
    ));
    tokio::spawn(db_worker(db_rx, pool, event_tx.clone()));

    loop {
        while let Ok(client) = register_rx.try_recv() {
            relays.insert(client.id, client.tx);
        }
        while let Ok(cmd) = command_rx.try_recv() {
            match cmd {
                AsyncCommand::SendPacket { client_id, packet } => {
                    if let Some(tx) = relays.get(&client_id) {
                        let _ = tx.send(packet).await.unwrap();
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
        connected_ips: ConnectedIps,
        limiter: RateLimiter,
        connection_semaphore: Arc<Semaphore>,
    ) -> Result<(), RuntimeError> {
        let port = settings::get_login_port()?;
        let addr = settings::get_bind_address()?;
        let bind = helpers::build_server_addr(addr, port);
        let ddos_mode_enabled = settings::get_ddos_mode()?;
        let listener: TcpListener = TcpListener::bind(bind).await?;
        loop {
            let connected_ips = connected_ips.clone();
            let connection_semaphore = connection_semaphore.clone();
            let event_tx = event_tx.clone();
            let limiter = limiter.clone();
            match listener.accept().await {
                Ok((stream, client_addr)) => {
                    let permit = match connection_semaphore.clone().try_acquire_owned() {
                        Ok(p) => p,
                        Err(_) => continue,
                    };
                    let ip = client_addr.ip().to_canonical();
                    if !ddos_protection::check_rate_limit(&limiter, ip) {
                        continue;
                    }
                    let inserted = {
                        let mut ips = connected_ips.lock().unwrap();
                        ips.insert(ip)
                    };
                    if ddos_mode_enabled && !inserted {
                        info!("Rejecting duplicate connection from {}", ip);
                        continue;
                    }
                    let client_id = crate::client::next_client_id();
                    let (tx, rx) = mpsc::channel::<Packet>(32);
                    register_tx.send(Register { id: client_id, tx }).await?;
                    event_tx
                        .send(AsyncEvent::ClientConnected {
                            client_id,
                            client_addr,
                        })
                        .unwrap();
                    tokio::spawn(async move {
                        let _permit = permit;
                        let _guard = IpGuard {
                            ip,
                            set: connected_ips,
                        };
                        match Runtime::new(stream).await {
                            Ok(runtime) => match runtime.run(client_id, event_tx.clone(), rx).await
                            {
                                Ok(_) => {
                                    event_tx
                                        .send(AsyncEvent::ClientDisconnected { client_id })
                                        .unwrap();
                                }
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
        event_tx: Sender<AsyncEvent>,
        register_tx: mpsc::Sender<Register>,
        mut transition_rx: mpsc::Receiver<AsyncCommand>,
        connected_ips: ConnectedIps,
        limiter: RateLimiter,
        connection_semaphore: Arc<Semaphore>,
    ) -> Result<(), RuntimeError> {
        let addr = settings::get_bind_address()?;
        let expected: Arc<Mutex<HashMap<i16, i32>>> = Arc::new(Mutex::new(HashMap::new()));
        let base_ports = inc::world::get_base_ports();
        let ddos_mode_enabled = settings::get_ddos_mode()?;
        for base_port in base_ports {
            let ports = inc::channel::get_channel_ports(base_port);
            for port in ports {
                let connected_ips = connected_ips.clone();
                let connection_semaphore = connection_semaphore.clone();
                let event_tx = event_tx.clone();
                let expected = expected.clone();
                let limiter = limiter.clone();
                let register_tx = register_tx.clone();
                let bind: SocketAddr = helpers::build_server_addr(addr.clone(), port);
                tokio::spawn(async move {
                    let listener = match TcpListener::bind(bind).await {
                        Ok(listener) => listener,
                        Err(e) => {
                            info!("Failed to bind {}: {}", port, e);
                            return;
                        }
                    };
                    loop {
                        match listener.accept().await {
                            Ok((stream, client_addr)) => {
                                dbg!("test");
                                let permit = match connection_semaphore.clone().try_acquire_owned()
                                {
                                    Ok(p) => p,
                                    Err(_) => continue,
                                };
                                dbg!("test");
                                let ip = client_addr.ip().to_canonical();
                                dbg!("test");
                                if !ddos_protection::check_rate_limit(&limiter, ip) {
                                    continue;
                                }
                                dbg!("test");
                                let inserted = {
                                    let mut ips = connected_ips.lock().unwrap();
                                    ips.insert(ip)
                                };
                                dbg!("test");
                                if ddos_mode_enabled && !inserted {
                                    info!("Rejecting duplicate connection from {}", ip);
                                    continue;
                                }
                                dbg!("test");
                                let client_id = expected.lock().unwrap().remove(&port);
                                if let Some(client_id) = client_id {
                                    let (tx, rx) = mpsc::channel::<Packet>(32);
                                    if register_tx
                                        .send(Register { id: client_id, tx })
                                        .await
                                        .is_err()
                                    {
                                        return;
                                    }
                                    event_tx
                                        .send(AsyncEvent::ClientConnected {
                                            client_id,
                                            client_addr,
                                        })
                                        .unwrap();
                                    let event_tx = event_tx.clone();
                                    let connected_ips = connected_ips.clone();
                                    tokio::spawn(async move {
                                        let _permit = permit;
                                        let _guard = IpGuard {
                                            ip,
                                            set: connected_ips,
                                        };
                                        match Runtime::new(stream).await {
                                            Ok(runtime) => {
                                                match runtime
                                                    .run(client_id, event_tx.clone(), rx)
                                                    .await
                                                {
                                                    Ok(_) => {
                                                        event_tx
                                                            .send(AsyncEvent::ClientDisconnected {
                                                                client_id,
                                                            })
                                                            .unwrap();
                                                    }
                                                    Err(e) => {
                                                        info!("Player runtime error: {}", e);
                                                        event_tx
                                                            .send(AsyncEvent::ClientDisconnected {
                                                                client_id,
                                                            })
                                                            .unwrap();
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                event_tx
                                                    .send(AsyncEvent::ClientDisconnected {
                                                        client_id,
                                                    })
                                                    .unwrap();
                                                info!("Player runtime init error: {}", e);
                                            }
                                        }
                                    });
                                }
                            }
                            Err(e) => {
                                info!("Player listener error: {}", e)
                            }
                        }
                    }
                });
            }
        }
        while let Some(AsyncCommand::AcceptTransition { client_id, port }) =
            transition_rx.recv().await
        {
            expected.lock().unwrap().insert(port, client_id);
        }
        Ok(())
    }
}
