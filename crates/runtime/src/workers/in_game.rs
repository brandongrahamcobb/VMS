use db::pool::DbPool;
use entity::character::model::CharacterModel;
use ipc::asyncronous::command::AsyncCommand;

use crate::workers::error::WorkerError;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub async fn in_game_worker(
    command_rx: Arc<Mutex<Receiver<AsyncCommand>>>,
    pool: &DbPool,
    event_tx: Sender<AsyncEvent>,
) -> Result<(), WorkerError> {
    loop {
        let cmd = { command_rx.lock().unwrap().try_recv() };
        match cmd {
            Ok(AsyncCommand::UpdateKeybindings { client_id, binds }) => {
                db::keybinding::setters::update_keybindings(pool, binds).await?;
            }
            Ok(AsyncCommand::UpdateStats {
                client_id,
                char_id,
                updates
            }) => {
                for stat in updates {
                    match stats_update {
                        StatsUpdate
                match stats_update
                let char_model =
                    db::character::getters::get_char_model_by_id(pool, char_id).await?;
                char_model.hp = hp;
                db::character::setters::update_characters(pool, vec![char_model]).await
            }
            Ok(_) => {
                tokio::task::yield_now().await;
                continue;
            }
        }
    }
}
