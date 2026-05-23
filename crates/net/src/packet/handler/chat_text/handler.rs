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

use action::scope::BroadcastScope;
use action::model::{Action, BroadcastAction};
use crate::packet::handler::chat_text::error::ChatTextError;
use crate::packet::handler::chat_text::reader::ChatTextReader;
use crate::packet::handler::chat_text::store::ChatTextStore;
use crate::packet::handler::result::HandlerResult;
use packet::model::Packet;
use db::pool::DbPool;
use session::model::Session;

pub struct ChatTextHandler;

impl ChatTextHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        pool: &DbPool,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, ChatTextError> {
        let reader: ChatTextReader = ChatTextReader::read_chat_text_packet(packet)?;
        let store: ChatTextStore = ChatTextStore::store_chat_text(pool, session, &reader).await?;
        let result: HandlerResult = self.build_chat_text_result(&store)?;
        Ok(result)
    }

    fn build_chat_text_result(
        &self,
        store: &ChatTextStore,
    ) -> Result<HandlerResult, ChatTextError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_chat_text_packet(store.admin, store.char_id, store.msg.clone(), store.show)?
            .finish();
        result.add_action(Action::Broadcast(BroadcastAction::Send {
            packet: packet.clone(),
            scope: BroadcastScope::Map {
                world_id: store.world_id,
                channel_id: store.channel_id,
                map_wz: store.map_wz,
            },
        }));
        Ok(result)
    }
}
