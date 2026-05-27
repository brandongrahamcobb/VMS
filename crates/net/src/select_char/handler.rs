/* select_char/handler.rs
 * The purpose of this module is to handle no-PIC, character selection.
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

use crate::result::HandlerResult;
use crate::select_char::error::SelectCharError;
use crate::select_char::reader::SelectCharReader;
use crate::select_char::store::SelectCharStore;
use action::model::{Action, SessionAction, SetAction};
use action::scope::SessionScope;
use packet::model::Packet;
use session::model::Session;
use state::model::SharedState;

pub struct SelectCharHandler;

impl Default for SelectCharHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectCharHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, SelectCharError> {
        let reader: SelectCharReader = SelectCharReader::read_select_char_packet(packet)?;
        let store: SelectCharStore =
            SelectCharStore::store_select_char(state, session, &reader).await?;
        let result: HandlerResult = self.build_select_char_result(&store)?;
        Ok(result)
    }

    fn build_select_char_result(
        &self,
        store: &SelectCharStore,
    ) -> Result<HandlerResult, SelectCharError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_select_char_packet(store.char_id, store.octets, store.port)?
            .finish();
        result.add_action(Action::Session(SessionAction::Set(SetAction::SetChar {
            char_id: store.char_id,
        })));
        result.add_action(Action::Session(SessionAction::Break {
            packet: packet.clone(),
            scope: SessionScope::Local,
        }));
        Ok(result)
    }
}
