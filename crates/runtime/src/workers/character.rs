use db::character::model::CharacterModel;
use db::item::model::ItemModel;
use db::pool::DbPool;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::store;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::tcp_event::AsyncEvent;

pub async fn character_worker(
    command_rx: Arc<Mutex<Receiver<AsyncCommand>>>,
    pool: DbPool,
    event_tx: Sender<AsyncEvent>,
) {
    loop {
        let cmd = { command_rx.lock().unwrap().try_recv() };
        match cmd {
            Ok(AsyncCommand::CreateChar {
                client_id,
                char_model,
            }) => {
                let char_model: CharacterModel =
                    db::character::setters::update_characters(&pool, vec![char_model]).await?;
                event_tx
                    .send(AsyncEvent::CharCreated {
                        client_id,
                        char_model,
                    })
                    .unwrap();
            }
            _ => {
                tokio::task::yield_now().await;
            }
        }
    }
}
