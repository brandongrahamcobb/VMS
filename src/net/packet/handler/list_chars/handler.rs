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

use crate::net::action::{Action, SetAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::list_chars::reader::ListCharsReader;
use crate::net::packet::handler::list_chars::store::ListCharsStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::Scope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ListCharsHandler;

impl ListCharsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: ListCharsReader = ListCharsReader::read_list_chars_packet(packet)?;
        let store: ListCharsStore =
            ListCharsStore::store_list_chars(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_list_chars_result(store.clone())?;
        Ok(result)
    }

    fn build_list_chars_result(
        &self,
        store: ListCharsStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_list_chars_packet(
                store.chars,
                store.channel.model.id,
                store.char_slots,
                store.pic_status,
            )?
            .finish();
        result.add_action(Action::Set(SetAction::SetChannel {
            channel: store.channel.clone(),
            scope: Scope::Local,
        }))?;
        result.add_action(Action::Set(SetAction::SetWorld {
            world: store.world.clone(),
            scope: Scope::Local,
        }))?;
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
