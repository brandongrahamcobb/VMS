use base::account::FailedCode;
use db::character::model::CharacterModel;
use db::pool::DbPool;
use ipc::syncronous;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::event::AsyncEvent;

use crate::workers::error::WorkerError;

pub async fn login_worker(
    command_rx: Arc<Mutex<Receiver<AsyncCommand>>>,
    pool: &DbPool,
    event_tx: Sender<AsyncEvent>,
) -> Result<(), WorkerError> {
    loop {
        let cmd = { command_rx.lock().unwrap().try_recv() };
        match cmd {
            Ok(AsyncCommand::LoginRequest {
                client_id,
                username,
                password,
            }) => {
                let event =
                    match db::account::getters::get_acc_model_by_username(pool, username.clone())
                        .await
                    {
                        Ok(acc_model) => {
                            let authenticated = syncronous::account::authenticate(
                                acc_model.password.clone(),
                                password.clone(),
                            );
                            match authenticated {
                                Ok(true) => {
                                    let status =
                                        syncronous::account::get_status_code_by_account(&acc_model);
                                    AsyncEvent::LoginSuccess {
                                        client_id,
                                        acc_model,
                                        status,
                                    }
                                }
                                Ok(false) => AsyncEvent::LoginFailed {
                                    client_id,
                                    code: FailedCode::InvalidCredentials,
                                },
                                Err(_) => AsyncEvent::LoginFailed {
                                    client_id,
                                    code: FailedCode::InvalidCredentials,
                                },
                            }
                        }
                        Err(_) => AsyncEvent::LoginFailed {
                            client_id,
                            code: FailedCode::UnknownCredentials,
                        },
                    };
                event_tx.send(event).unwrap();
            }
            Ok(AsyncCommand::ListChars {
                client_id,
                acc_id,
                channel_id,
                world_id,
            }) => {
                let slots: i16 = db::character::getters::get_char_max_by_account_and_world_id(
                    pool, acc_id, world_id,
                )
                .await
                .unwrap_or(8);
                let char_models: Vec<CharacterModel> =
                    db::character::getters::get_char_models_by_acc_id_and_world_id(
                        pool, acc_id, world_id,
                    )
                    .await?;
                let event = AsyncEvent::ListCharsSuccess {
                    client_id,
                    channel_id,
                    char_models,
                    slots,
                    world_id,
                };
                event_tx.send(event).unwrap();
            }
            Ok(AsyncCommand::SetTosAccepted { client_id, acc_id }) => {
                std::hint::black_box(client_id);
                db::account::setters::accept_tos_by_acc_id(pool, acc_id).await?;
            }
            Ok(AsyncCommand::CheckCharName { client_id, ign }) => {
                let exists = db::character::getters::get_char_model_by_name(pool, ign.clone())
                    .await
                    .is_ok();
                let event = AsyncEvent::CheckCharName {
                    client_id,
                    exists,
                    ign,
                };
                event_tx.send(event).unwrap();
            }
            Ok(AsyncCommand::SelectCharWithPic {
                client_id,
                acc_id,
                char_id,
                mac,
                hwid,
                pic,
            }) => {
                std::hint::black_box(mac);
                std::hint::black_box(hwid);
                let Ok(acc_model) = db::account::getters::get_acc_model_by_id(&pool, acc_id).await
                else {
                    continue;
                };
                let status = syncronous::account::check_pic(acc_model.pic, pic);
                let event = AsyncEvent::SelectCharWithPic {
                    client_id,
                    char_id,
                    status,
                };
                event_tx.send(event).unwrap();
            }
            Ok(_) => {
                tokio::task::yield_now().await;
                continue;
            }
            Err(_) => {
                tokio::task::yield_now().await;
                continue;
            }
        }
    }
}
