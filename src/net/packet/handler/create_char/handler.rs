/* create_char/handler.rs
 * The purpose of this module is to handle character creation.
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
use crate::net::packet::handler::create_char::error::CreateCharError;
use crate::net::packet::handler::create_char::reader::CreateCharReader;
use crate::net::packet::handler::create_char::store::CreateCharStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::Scope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct CreateCharHandler;

impl CreateCharHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, CreateCharError> {
        let reader: CreateCharReader = CreateCharReader::read_create_character_packet(packet)?;
        let store: CreateCharStore =
            CreateCharStore::store_create_char(state, session, &reader).await?;
        let result: HandlerResult = self.build_create_char_result(&store)?;
        Ok(result)
    }

    fn build_create_char_result(
        &self,
        store: &CreateCharStore,
    ) -> Result<HandlerResult, CreateCharError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_create_char_packet(&store.char)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        });
        Ok(result)
    }
}
