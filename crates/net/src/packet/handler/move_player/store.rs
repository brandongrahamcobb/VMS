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

use crate::packet::handler::move_player::error::MovePlayerError;
use crate::packet::handler::move_player::reader::MovePlayerReader;
use entity::map;
use entity::map::model::Point;
use session::model::Session;
use state::model::SharedState;

pub struct MovePlayerStore {
    pub char_id: i32,
    pub empty: bool,
    pub movement_bytes: Vec<u8>,
    pub too_short: bool,
    pub world_id: i16,
    pub channel_id: u8,
    pub map_wz: i32,
}

impl MovePlayerStore {
    pub async fn store_move_player(
        state: &SharedState,
        session: &Session,
        reader: &MovePlayerReader,
    ) -> Result<Self, MovePlayerError> {
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = session.get_channel_id()?;
        let map_wz: i32 = session.get_map_wz()?;
        let char_id: i32 = session.get_char_id()?;
        let pos: Point =
            map::service::parse_position(&reader.movement_bytes).unwrap_or(Point { x: 0, y: 0 });
        {
            let state = state.lock().await;
            state
                .with_mut_map(world_id, channel_id, map_wz, |map| {
                    if let Some(char) = map.chars.get_mut(&char_id) {
                        char.pos = pos;
                    }
                })
                .await?;
        }
        Ok(Self {
            char_id: session.get_char_id()?,
            empty: reader.empty,
            movement_bytes: reader.movement_bytes.clone(),
            too_short: reader.too_short,
            world_id,
            channel_id,
            map_wz,
        })
    }
}
