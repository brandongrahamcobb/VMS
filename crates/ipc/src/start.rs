use net::packet::tcp::{LoginServer, PlayerServer};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use tracing::info;

use crate::channel::{TcpCommand, TcpEvent};

pub async fn start_server(
    event_tx: Sender<TcpEvent>,
    command_rx: Receiver<TcpCommand>,
    db: DbPool,
) -> Result<(), Box<IPCError>> {
    let command_rx = Arc::new(Mutex::new(command_rx));
    tokio::spawn({
        let command_tx = Arc::clone(&command_rx);
        let db = db.clone();
        let event_tx = event_tx.clone();
        async move {
            login_worker(command_rx, db, event_tx).await;
        }
    });

    info!("Binding to login server...");
    let login = LoginServer::run(event_tx.clone(), Arc::clone(&command_rx), login_tx);

    info!("Binding to player server...");
    let player = PlayerServer::run(event_tx.clone(), Arc::clone(&command_rx));

    let (_, _) = try_join!(login, player).map_err(Box::new)?;
    Ok(())
}
