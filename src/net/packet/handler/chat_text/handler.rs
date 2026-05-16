/* chat_text/handler.rs
 * The purpose of this module is to handle general chats.
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
use crate::net::packet::handler::chat_text::error::ChatTextError;
use crate::net::packet::handler::chat_text::reader::ChatTextReader;
use crate::net::packet::handler::chat_text::store::ChatTextStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{MapScope, Scope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ChatTextHandler;

impl ChatTextHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, ChatTextError> {
        let reader: ChatTextReader = ChatTextReader::read_chat_text_packet(packet)?;
        let store: ChatTextStore =
            ChatTextStore::store_chat_text(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_chat_text_result(store.clone())?;
        Ok(result)
    }

    fn build_chat_text_result(&self, store: ChatTextStore) -> Result<HandlerResult, ChatTextError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_chat_text_packet(store.admin, store.char_id, store.msg.clone(), store.show)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        });
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        });
        Ok(result)
    }
}
