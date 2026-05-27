/* credentials/handler.rs
 * The purpose of this module is to handle credentials validation.
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

use crate::credentials::error::CredentialsError;
use crate::credentials::reader::CredentialsReader;
use crate::credentials::store::CredentialsStore;
use crate::result::HandlerResult;
use action::model::{Action, SessionAction, SetAction};
use action::scope::SessionScope;
use entity::account::wrapper::StatusCode;
use packet::model::Packet;
use session::model::Session;
use state::model::SharedState;

pub struct CredentialsHandler;

impl Default for CredentialsHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, CredentialsError> {
        let reader: CredentialsReader = CredentialsReader::read_credentials_packet(packet)?;
        let mut store: CredentialsStore =
            CredentialsStore::store_credentials(state, session, &reader).await?;
        let result: HandlerResult = self.build_credentials_result(&mut store)?;
        Ok(result)
    }

    fn build_credentials_result(
        &self,
        store: &mut CredentialsStore,
    ) -> Result<HandlerResult, CredentialsError> {
        let mut result: HandlerResult = HandlerResult::new();
        match store.status.clone() {
            StatusCode::Failed(code) => {
                let code = code as i16;
                let packet: Packet = Packet::new_empty()
                    .build_credentials_handler_failed_login_packet(code)?
                    .finish();
                result.add_action(Action::Session(SessionAction::Send {
                    packet: packet.clone(),
                    scope: SessionScope::Local,
                }));
            }
            StatusCode::Pending(code) => {
                let acc = store.acc.take().unwrap();
                let code = code as i16;
                let packet: Packet = Packet::new_empty()
                    .build_credentials_handler_failed_login_packet(code)?
                    .finish();
                result.add_action(Action::Session(SessionAction::Set(SetAction::SetAccount {
                    acc_id: acc.model.get_id()?,
                })));
                result.add_action(Action::Session(SessionAction::Send {
                    packet: packet.clone(),
                    scope: SessionScope::Local,
                }));
            }
            StatusCode::Success(_) => {
                let acc = store.acc.take().unwrap();
                let packet: Packet = Packet::new_empty()
                    .build_credentials_handler_successful_login_packet(&acc)?
                    .finish();
                result.add_action(Action::Session(SessionAction::Set(SetAction::SetAccount {
                    acc_id: acc.model.get_id()?,
                })));
                result.add_action(Action::Session(SessionAction::Send {
                    packet: packet.clone(),
                    scope: SessionScope::Local,
                }));
            }
        }
        Ok(result)
    }
}
