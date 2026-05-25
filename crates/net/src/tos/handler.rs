/* tos/handler.rs
 * The purpose of this module is to handle Terms of Service acceptance.
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

use action::model::{Action, SessionAction};
use crate::result::HandlerResult;
use crate::tos::error::TosError;
use crate::tos::reader::TosReader;
use crate::tos::store::TosStore;
use packet::model::Packet;
use db::pool::DbPool;
use action::scope::SessionScope;
use session::model::Session;

pub struct TosHandler;

impl TosHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        pool: &DbPool,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, TosError> {
        let reader: TosReader = TosReader::read_tos_packet(packet)?;
        let store: TosStore = TosStore::store_tos(pool, session, &reader).await?;
        let result: HandlerResult = self.build_tos_result(&store)?;
        Ok(result)
    }

    fn build_tos_result(&self, store: &TosStore) -> Result<HandlerResult, TosError> {
        let mut result: HandlerResult = HandlerResult::new();
        if store.accepted {
            let packet: Packet = Packet::new_empty()
                .build_credentials_handler_successful_login_packet(&store.acc)?
                .finish();
            result.add_action(Action::Session(SessionAction::Send {
                packet: packet.clone(),
                scope: SessionScope::Local,
            }));
        }
        Ok(result)
    }
}
