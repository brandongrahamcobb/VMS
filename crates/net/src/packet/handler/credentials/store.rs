/* credentials/store.rs
 * The purpose of this module is to resolve relevant variables for credentials validation.
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

use crate::packet::handler::credentials::error::CredentialsError;
use crate::packet::handler::credentials::reader::CredentialsReader;
use db::pool::DbPool;
use entity::account;
use entity::account::wrapper::Account;
use entity::account::wrapper::{FailedCode, StatusCode};
use session::model::Session;
use state::model::SharedState;

pub struct CredentialsStore {
    pub acc: Option<Account>,
    pub status: StatusCode,
}

impl CredentialsStore {
    pub async fn store_credentials(
        state: &SharedState,
        session: &Session,
        reader: &CredentialsReader,
    ) -> Result<Self, CredentialsError> {
        let pool: DbPool = state.lock().await.db.clone();
        match assembly::account::assemble::assemble_acc_by_username(&pool, reader.username.clone())
            .await
        {
            Ok(acc) => {
                let status = if account::service::authenticate(
                    acc.model.password.clone(),
                    reader.pw.clone(),
                )? {
                    let sessions = {
                        let state = state.lock().await;
                        state.sessions.get_all(session.id)
                    };
                    let mut acc_ids: Vec<i32> = Vec::<i32>::new();
                    for s in sessions {
                        acc_ids.push(s.get_acc_id()?);
                    }
                    account::service::get_status_code_by_account(acc_ids.clone(), acc.model.clone())
                        .await?
                } else {
                    StatusCode::Failed(FailedCode::InvalidCredentials)
                };
                Ok(Self {
                    acc: Some(acc),
                    status,
                })
            }
            Err(_) => {
                let status = StatusCode::Failed(FailedCode::UnknownCredentials);
                Ok(Self { acc: None, status })
            }
        }
    }
}
