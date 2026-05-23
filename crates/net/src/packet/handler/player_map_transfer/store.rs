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

use crate::packet::handler::player_map_transfer::error::PlayerMapTransferError;
use db::pool::DbPool;
use entity::character::wrapper::Character;
use session::model::Session;
use state::model::SharedState;

pub struct PlayerMapTransferStore {
    pub char: Character,
    pub world_id: i16,
    pub channel_id: u8,
    pub map_wz: i32,
}

impl PlayerMapTransferStore {
    pub async fn store_player_map_transfer(
        state: &SharedState,
        session: &Session,
    ) -> Result<Self, PlayerMapTransferError> {
        let state = state.lock().await;
        state.sessions.update(session.id, |session| {
            session.transitioning = false;
        });
        let pool: DbPool = state.db.clone();
        let char_id: i32 = session.get_char_id()?;
        let char: Character =
            assembly::character::assemble::assemble_char_by_id(&pool, char_id).await?;
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = session.get_channel_id()?;
        let map_wz: i32 = session.get_map_wz()?;
        Ok(Self {
            char,
            world_id,
            channel_id,
            map_wz,
        })
    }
}
