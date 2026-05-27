/* register_pic/handler.rs
 * The purpose of this module is to handle PIC registration.
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

use crate::register_pic::error::RegisterPicError;
use crate::register_pic::reader::RegisterPicReader;
use crate::register_pic::store::RegisterPicStore;
use crate::result::HandlerResult;
use action::model::{Action, SessionAction, SetAction};
use action::scope::SessionScope;
use packet::model::Packet;
use session::model::Session;
use state::model::SharedState;

pub struct RegisterPicHandler;

impl Default for RegisterPicHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RegisterPicHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, RegisterPicError> {
        let reader: RegisterPicReader = RegisterPicReader::read_register_pic_packet(packet)?;
        let store: RegisterPicStore =
            RegisterPicStore::store_register_pic(state, session, &reader).await?;
        let result = self.build_register_pic_result(&store)?;
        Ok(result)
    }

    fn build_register_pic_result(
        &self,
        store: &RegisterPicStore,
    ) -> Result<HandlerResult, RegisterPicError> {
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
