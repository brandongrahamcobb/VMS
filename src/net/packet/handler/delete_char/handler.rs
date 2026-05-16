/* delete_char/handler.rs
 * The purpose of this module is to handle character deletion.
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

use crate::net::action::Action;
use crate::net::packet::handler::delete_char::error::DeleteCharError;
use crate::net::packet::handler::delete_char::reader::DeleteCharReader;
use crate::net::packet::handler::delete_char::store::DeleteCharStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::Scope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct DeleteCharHandler;

impl DeleteCharHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, DeleteCharError> {
        let reader: DeleteCharReader = DeleteCharReader::read_delete_char_packet(packet)?;
        let store: DeleteCharStore =
            DeleteCharStore::store_delete_char(state, session.clone(), reader).await?;
        let result: HandlerResult = self.build_delete_char_result(store)?;
        Ok(result)
    }

    fn build_delete_char_result(
        &self,
        store: DeleteCharStore,
    ) -> Result<HandlerResult, DeleteCharError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet = Packet::new_empty()
            .build_delete_char_packet(store.char_id, store.pic_status)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        });
        Ok(result)
    }
}
