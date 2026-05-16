/* move_player/store.rs
 * The purpose of this module is to resolve relevant variables for player movement.
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

use crate::net::packet::handler::move_player::error::MovePlayerError;
use crate::net::packet::handler::move_player::reader::MovePlayerReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct MovePlayerStore {
    pub char_id: i32,
    pub empty: bool,
    pub movement_bytes: Vec<u8>,
    pub too_short: bool,
}

impl MovePlayerStore {
    pub async fn store_move_player(
        state: &SharedState,
        session: Session,
        reader: MovePlayerReader,
    ) -> Result<Self, MovePlayerError> {
        std::hint::black_box(state);
        Ok(Self {
            char_id: session.get_char_id()?,
            empty: reader.empty,
            movement_bytes: reader.movement_bytes.clone(),
            too_short: reader.too_short,
        })
    }
}
