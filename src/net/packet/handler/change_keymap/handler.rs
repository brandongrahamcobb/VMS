/* change_keymap/handler.rs
 * The purpose of this module is to handle keymap changes.
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

use crate::net::packet::handler::change_keymap::error::ChangeKeymapError;
use crate::net::packet::handler::change_keymap::reader::ChangeKeymapReader;
use crate::net::packet::handler::change_keymap::store::ChangeKeymapStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ChangeKeymapHandler;

impl ChangeKeymapHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, ChangeKeymapError> {
        let reader: ChangeKeymapReader = ChangeKeymapReader::read_change_keymap_packet(packet)?;
        let store: ChangeKeymapStore =
            ChangeKeymapStore::store_change_keymap(state, session.clone(), reader).await?;
        let result: HandlerResult = self.build_change_keymap_result(store)?;
        Ok(result)
    }

    fn build_change_keymap_result(
        &self,
        store: ChangeKeymapStore,
    ) -> Result<HandlerResult, ChangeKeymapError> {
        // no packet neccessary
        std::hint::black_box(store);
        let result: HandlerResult = HandlerResult::new();
        Ok(result)
    }
}
