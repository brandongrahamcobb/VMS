/* list_chars/handler.rs
 * The purpose of this module is to handle character listing.
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

use action::scope::SessionScope;
use action::model::{Action, SessionAction, SetAction};
use crate::packet::handler::list_chars::error::ListCharsError;
use crate::packet::handler::list_chars::reader::ListCharsReader;
use crate::packet::handler::list_chars::store::ListCharsStore;
use crate::packet::handler::result::HandlerResult;
use packet::model::Packet;
use db::pool::DbPool;
use session::model::Session;

pub struct ListCharsHandler;

impl ListCharsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        pool: &DbPool,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, ListCharsError> {
        let reader: ListCharsReader = ListCharsReader::read_list_chars_packet(packet)?;
        let store: ListCharsStore =
            ListCharsStore::store_list_chars(pool, session, &reader).await?;
        let result: HandlerResult = self.build_list_chars_result(&store)?;
        Ok(result)
    }

    fn build_list_chars_result(
        &self,
        store: &ListCharsStore,
    ) -> Result<HandlerResult, ListCharsError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_list_chars_packet(
                &store.chars,
                store.channel_id,
                store.char_slots,
                store.pic_status,
            )?
            .finish();
        result.add_action(Action::Session(SessionAction::Set(SetAction::SetChannel {
            channel_id: store.channel_id,
            scope: SessionScope::Local,
        })));
        result.add_action(Action::Session(SessionAction::Set(SetAction::SetWorld {
            world_id: store.world_id,
            scope: SessionScope::Local,
        })));
        result.add_action(Action::Session(SessionAction::Send {
            packet: packet.clone(),
            scope: SessionScope::Local,
        }));
        Ok(result)
    }
}
