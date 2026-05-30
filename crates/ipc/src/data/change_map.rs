/* change_map/store.rs
 * The purpose of this module is to resolve relevant variables when changing maps.
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

use crate::change_map::error::ChangeMapEntityError;
use crate::change_map::reader::ChangeMapReader;
use db::pool::DbPool;
use entity::character::wrapper::Character;
use session::model::Session;
use state::model::SharedState;

pub struct ChangeMapEvent {
    pub after_map_wz: i32,
    pub channel_id: u8,
    pub char: Character,
    pub died: i16,
    pub pid: u8,
    pub wheel_of_destiny: i16,
}

pub struct ChangeMapCommand {
    pub client_id: i32,
    pub char_id: i32,
    pub died: i16,
    pub target_map: i32,
    pub target_name: String,
    pub wheel_of_destiny: i16,
}

impl ChangeMapEvent {
    pub async fn store_change_map(
        state: &SharedState,
        session: &Session,
        reader: &ChangeMapReader,
    ) -> Result<Self, ChangeMapEntityError> {
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = session.get_channel_id()?;
        let map_wz: i32 = session.get_map_wz()?;
        let (tm, pid): (i32, u8) = {
            let state = state.lock().await;
            let (tm, pid) = state
                .with_map(world_id, channel_id, map_wz, |map| {
                    match map.get_portal(reader.tn.clone()) {
                        Ok(p) => (p.info.tm, p.info.pid),
                        Err(_) => (map_wz, 0),
                    }
                })
                .await?;
            state.sessions.update(session.id, |session| {
                session.transitioning = true;
            });
            (tm, pid)
        };
        let char_id: i32 = session.get_char_id()?;
        let pool: DbPool = state.lock().await.db.clone();
        let char: Character =
            assembly::character::assemble::assemble_char_by_id(&pool, char_id).await?;
        Ok(Self {
            after_map_wz: tm,
            channel_id,
            char,
            died: reader.died,
            pid,
            wheel_of_destiny: reader.wod,
        })
    }
}
