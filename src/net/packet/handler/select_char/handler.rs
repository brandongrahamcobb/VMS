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

use crate::net::action::{Action, SetAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::select_char::reader::SelectCharReader;
use crate::net::packet::handler::select_char::store::SelectCharStore;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::Scope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct SelectCharHandler;

impl SelectCharHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: SelectCharReader = SelectCharReader::read_select_char_packet(packet)?;
        let store: SelectCharStore =
            SelectCharStore::store_select_char(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_select_char_result(store.clone())?;
        Ok(result)
    }

    fn build_select_char_result(
        &self,
        store: SelectCharStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_select_char_packet(
                store.channel.clone(),
                store.char.clone(),
                store.octets,
            )?
            .finish();
        result.add_action(Action::Set(SetAction::SetChar {
            char: store.char.clone(),
        }))?;
        result.add_action(Action::Break {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
