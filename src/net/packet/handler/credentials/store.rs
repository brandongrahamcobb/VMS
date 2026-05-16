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

use crate::models::account;
use crate::models::account::wrapper::Account;
use crate::models::account::wrapper::{FailedCode, StatusCode};
use crate::net::packet::handler::credentials::error::CredentialsError;
use crate::net::packet::handler::credentials::reader::CredentialsReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

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
        match account::service::get_account_by_username(state, reader.username.clone()).await {
            Ok(acc) => {
                let status = if acc.authenticate(reader.pw.clone())? {
                    let sessions = {
                        let state = state.lock().await;
                        state.sessions.get_all(session.id)
                    };
                    let mut acc_ids: Vec<i32> = Vec::<i32>::new();
                    for s in sessions {
                        acc_ids.push(s.get_acc_id()?);
                    }
                    acc.get_status_code_by_account(acc_ids.clone()).await?
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
