use assembly;
use bcrypt::verify;
use db::pool::DbPool;
use entity::account::model::AccountModel;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use crate::tcp_command::TcpCommand;
use crate::tcp_event::TcpEvent;

#[derive(Clone)]
pub enum StatusCode {
    Failed(FailedCode),
    Pending(PendingCode),
    Success(SuccessCode),
}

#[derive(Clone)]
pub enum PendingCode {
    PendingTOS = 23,
}

#[derive(Clone)]
pub enum SuccessCode {
    Success = 0,
}

#[derive(Clone)]
pub enum FailedCode {
    Banned = 2,
    InvalidCredentials = 4,
    UnknownCredentials = 5,
    Playing = 7,
}

pub async fn login_worker(
    command_rx: Arc<Mutex<Receiver<TcpCommand>>>,
    pool: &DbPool,
    event_tx: Sender<TcpEvent>,
) -> () {
    loop {
        let cmd = { command_rx.lock().unwrap().try_recv() };
        match cmd {
            Ok(TcpCommand::CheckCredentials {
                client_id,
                username,
                password,
            }) => {
                let event = match assembly::account::assemble::assemble_acc_by_username(
                    pool,
                    username.clone(),
                )
                .await
                {
                    Ok(acc) => {
                        let authenticated = domain::account::authenticate(
                            acc.model.password.clone(),
                            password.clone(),
                        );
                        match authenticated {
                            Ok(true) => {
                                let status =
                                    domain::account::get_status_code_by_account(&acc.model);
                                TcpEvent::LoginSuccess {
                                    client_id,
                                    acc,
                                    status,
                                }
                            }
                            Ok(false) => TcpEvent::LoginFailed {
                                client_id,
                                code: FailedCode::InvalidCredentials,
                            },
                            Err(_) => TcpEvent::LoginFailed {
                                client_id,
                                code: InvalidCredentials,
                            },
                        }
                    }
                    Err(_) => TcpEvent::LoginFailed {
                        client_id,
                        code: FailedCode::UnknownCredentials,
                    },
                };
                event_tx.send(event).unwrap();
            }
            Ok(TcpCommand::ListChars {
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
                let event = domain::login::list_chars(pool, acc_id, world_id)
                    .await
                    .map(|chars| TcpEvent::ListCharsSuccess {
                        client_id,
                        channel_id,
                        chars,
                        slots,
                        world_id,
                    })
                    .unwrap_or(TcpEvent::ListCharsFailed { client_id });
                event_tx.send(event).unwrap();
            }
            Ok(TcpCommand::SetTosAccepted { client_id, acc_id }) => {
                db::account::setters::accept_tos_by_acc_id(pool, acc_id).await?;
            }
            Ok(TcpCommand::CheckCharName { client_id, ign }) => {
                let Some(_) = db::character::getters::get_char_model_by_name(pool, ign).await
                else {
                    continue;
                };
                let event = TcpEvent::CheckCharName {
                    client_id,
                    exists,
                    ign,
                };
                event_tx.send(event).unwrap();
            }
            Ok(TcpCommand::SelectCharWithPic {
                client_id,
                acc_id,
                char_id,
                mac,
                hwid,
                pic,
            }) => {
                let Some(acc_model): Option<AccountModel> =
                    db::account::getters::get_acc_model_by_id(&pool, acc_id).await
                else {
                    continue;
                };
                let status = domain::account::check_pic(acc_model.pic, pic);
                let event = TcpEvent::SelectCharWithPic {
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
        }
    }
}
