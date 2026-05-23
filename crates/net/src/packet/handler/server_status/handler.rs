/* server_status/handler.rs
 * The purpose of this module is to handle server status updates.
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

use crate::action::model::{Action, SessionAction};
use crate::action::scope::SessionScope;
use crate::packet::handler::result::HandlerResult;
use crate::packet::handler::server_status::error::ServerStatusError;
use crate::packet::handler::server_status::store::ServerStatusStore;
use packet::model::Packet;
use state::model::SharedState;

pub struct ServerStatusHandler;

impl ServerStatusHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(&self, state: &SharedState) -> Result<HandlerResult, ServerStatusError> {
        let store: ServerStatusStore = ServerStatusStore::store_server_status(state).await?;
        let result: HandlerResult = self.build_server_status_result(&store)?;
        Ok(result)
    }

    fn build_server_status_result(
        &self,
        store: &ServerStatusStore,
    ) -> Result<HandlerResult, ServerStatusError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_server_status_packet(store.status)?
            .finish();
        result.add_action(Action::Session(SessionAction::Send {
            packet: packet.clone(),
            scope: SessionScope::Local,
        }));
        Ok(result)
    }
}
