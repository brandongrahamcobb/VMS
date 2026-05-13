/* party_search/handler.rs
 * The purpose of this module is to handle party searching.
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

use crate::net::error::NetworkError;
use crate::net::packet::handler::party_search::reader::PartySearchReader;
use crate::net::packet::handler::party_search::store::PartySearchStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct PartySearchHandler;

impl PartySearchHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: PartySearchReader = PartySearchReader::read_party_search_packet(packet)?;
        let store: PartySearchStore =
            PartySearchStore::store_party_search(state, session, reader.clone())?;
        let result: HandlerResult = self.build_party_search_result(store.clone())?;
        Ok(result)
    }

    fn build_party_search_result(
        &self,
        store: PartySearchStore,
    ) -> Result<HandlerResult, NetworkError> {
        // Not implemented
        std::hint::black_box(store);
        let result: HandlerResult = HandlerResult::new();
        Ok(result)
    }
}
