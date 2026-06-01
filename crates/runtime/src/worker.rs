/* runtime/src/workers/login.rs
 * The purpose of this module is to handle asyncronous commands during login.
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

use base::{account::FailedCode, character::StatsUpdate};
use db::character::model::CharacterModel;
use db::pool::DbPool;
use ipc::{asyncronous::db_command::DatabaseCommand, syncronous};
use std::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;

use ipc::asyncronous::event::AsyncEvent;

use crate::error::RuntimeError;

pub async fn db_worker(
    mut db_rx: Receiver<DatabaseCommand>,
    pool: DbPool,
    event_tx: Sender<AsyncEvent>,
) -> Result<(), RuntimeError> {
    loop {
        let cmd = { db_rx.try_recv() };
        match cmd {
            Ok(DatabaseCommand::LoginRequest {
                client_id,
                username,
                password,
            }) => {
                let event =
                    match db::account::getters::get_acc_model_by_username(&pool, username.clone())
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
            Ok(DatabaseCommand::ListChars {
                client_id,
                acc_id,
                channel_id,
                world_id,
            }) => {
                let slots: i16 = db::character::getters::get_char_max_by_account_and_world_id(
                    &pool, acc_id, world_id,
                )
                .await
                .unwrap_or(8);
                let char_models: Vec<CharacterModel> =
                    db::character::getters::get_char_models_by_acc_id_and_world_id(
                        &pool, acc_id, world_id,
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
            Ok(DatabaseCommand::SetTosAccepted { client_id, acc_id }) => {
                std::hint::black_box(client_id);
                db::account::setters::accept_tos_by_acc_id(&pool, acc_id).await?;
            }
            Ok(DatabaseCommand::CheckCharName { client_id, ign }) => {
                let exists = db::character::getters::get_char_model_by_name(&pool, ign.clone())
                    .await
                    .is_ok();
                let event = AsyncEvent::CheckCharName {
                    client_id,
                    exists,
                    ign,
                };
                event_tx.send(event).unwrap();
            }
            Ok(DatabaseCommand::SelectCharWithPic {
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
            Ok(DatabaseCommand::CreateCharRequest {
                client_id,
                char_model,
            }) => {
                let char_models: Vec<CharacterModel> =
                    db::character::setters::update_characters(&pool, vec![char_model]).await?;
                event_tx
                    .send(AsyncEvent::CharCreated {
                        client_id,
                        char_model: char_models[0].clone(),
                    })
                    .unwrap();
            }
            Ok(DatabaseCommand::UpdateKeybindings { client_id, binds }) => {
                std::hint::black_box(client_id);
                db::keybinding::setters::update_keybindings(&pool, binds).await?;
            }
            Ok(DatabaseCommand::UpdateStats {
                client_id,
                char_id,
                updates,
            }) => {
                std::hint::black_box(client_id);
                let mut char_model =
                    db::character::getters::get_char_model_by_id(&pool, char_id).await?;
                for stat in updates {
                    match stat {
                        StatsUpdate::Exp { exp } => char_model.exp = exp,
                        StatsUpdate::Str { strength } => char_model.strength = strength,
                        StatsUpdate::Dex { dexterity } => char_model.dexterity = dexterity,
                        StatsUpdate::Luk { luck } => char_model.luck = luck,
                        StatsUpdate::Int { intelligence } => char_model.intelligence = intelligence,
                        StatsUpdate::Health { hp } => char_model.hp = hp,
                        StatsUpdate::Mana { mp } => char_model.mp = mp,
                        StatsUpdate::MaxHealth { max_hp } => char_model.max_hp = max_hp,
                        StatsUpdate::MaxMana { max_mp } => char_model.max_mp = max_mp,
                        StatsUpdate::AbilityPoints { ap } => char_model.ap = ap,
                        StatsUpdate::SkillPoints { sp } => char_model.sp = sp,
                    }
                }
                db::character::setters::update_characters(&pool, vec![char_model]).await?;
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
