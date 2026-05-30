use db::pool::DbPool;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::store;
use crate::tcp_command::TcpCommand;
use crate::tcp_event::TcpEvent;

pub async fn character_worker(
    command_rx: Arc<Mutex<Receiver<TcpCommand>>>,
    db: DbPool,
    event_tx: Sender<TcpEvent>,
) {
    loop {
        let cmd = { command_rx.lock().unwrap().try_recv() };
        match cmd {
            Ok(TcpCommand::CreateCharacter {
                client_id,
                acc_id,
                name,
            }) => P
                let event = domain::account::create_char(&db, acc_id)
            match assembly::character::assemble::create(&db, acc_id, name).await {
                Ok(character) => event_tx
                    .send(TcpEvent::CharacterCreated {
                        client_id,
                        character,
                    })
                    .unwrap(),
                Err(_) => event_tx
                    .send(TcpEvent::CharacterCreateFailed { client_id })
                    .unwrap(),
            },
            Ok(TcpCommand::DeleteCharacter { client_id, char_id }) => {
                match assembly::character::assemble::delete(&db, char_id).await {
                    Ok(_) => event_tx
                        .send(TcpEvent::CharacterDeleted { client_id, char_id })
                        .unwrap(),
                    Err(_) => event_tx
                        .send(TcpEvent::CharacterDeleteFailed { client_id })
                        .unwrap(),
                }
            }
            Ok(TcpCommand::CheckCharacterName { client_id, name }) => {
                match assembly::character::assemble::name_exists(&db, &name).await {
                    Ok(true) => event_tx
                        .send(TcpEvent::CharacterNameTaken { client_id })
                        .unwrap(),
                    Ok(false) => event_tx
                        .send(TcpEvent::CharacterNameAvailable { client_id })
                        .unwrap(),
                    Err(_) => event_tx
                        .send(TcpEvent::CharacterNameTaken { client_id })
                        .unwrap(),
                }
            }
            _ => {
                tokio::task::yield_now().await;
            }
        }
    }
}
