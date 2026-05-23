/* party_search/store.rs
 * The purpose of this module is to resolve relevant variables for party searching.
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

use crate::packet::handler::party_search::error::PartySearchError;
use crate::packet::handler::party_search::reader::PartySearchReader;
use session::model::Session;
use state::model::SharedState;

pub struct PartySearchStore;

impl PartySearchStore {
    pub fn store_party_search(
        state: &SharedState,
        session: &Session,
        reader: &PartySearchReader,
    ) -> Result<Self, PartySearchError> {
        std::hint::black_box(state);
        std::hint::black_box(session);
        std::hint::black_box(reader);
        Ok(Self)
    }
}
