use db::pool::DbPool;
use entity::character::model::CharacterModel;

use crate::{tcp_command::TcpCommand, tcp_event::TcpEvent};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub async fn in_game_worker(
    command_rx: Arc<Mutex<Receiver<TcpCommand>>>,
    pool: &DbPool,
    event_tx: Sender<TcpEvent>,
) -> () {
    loop {
        let cmd = { command_rx.lock().unwrap().try_recv() };
        match cmd {
            Ok(TcpCommand::UpdateKeybindings { client_id, binds }) => {
                let Some(_) = db::keybinding::setters::update_keybindings(pool, binds).await else {
                    continue;
                };
            }
            Ok(TcpCommand::UpdateHealth {
                client_id,
                char_id,
                hp,
            }) => {
                let Some(char_model): CharacterModel =
                    db::character::getters::get_char_model_by_id(pool, char_id).await
                else {
                    continue;
                };
                char_model.hp = hp;
                let Some(_) =
                    db::character::setters::update_characters(pool, vec![char_model]).await
                else {
                    continue;
                };
            }
            Ok(_) => {
                tokio::task::yield_now().await;
                continue;
            }
        }
    }
}
