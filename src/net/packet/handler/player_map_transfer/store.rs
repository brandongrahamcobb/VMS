/* player_map_transfer/store.rs
 * The purpose of this module is to resolve relevant variables for player map transfers.
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

use crate::models::character;
use crate::models::character::wrapper::Character;
use crate::net::packet::handler::player_map_transfer::error::PlayerMapTransferError;
use crate::net::packet::handler::player_map_transfer::reader::PlayerMapTransferReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct PlayerMapTransferStore {
    pub char: Character,
}

impl PlayerMapTransferStore {
    pub async fn store_player_map_transfer(
        state: &SharedState,
        session: &Session,
        reader: &PlayerMapTransferReader,
    ) -> Result<Self, PlayerMapTransferError> {
        {
            let state = state.lock().await;
            state.sessions.update(session.id, |session| {
                session.map_lock = false;
            });
        }
        let char_id: i32 = session.get_char_id()?;
        let char: Character = character::service::get_char_by_id(state, char_id).await?;
        std::hint::black_box(reader);
        Ok(Self { char })
    }
}
