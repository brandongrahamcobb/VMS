/* cc/store.rs
 * The purpose of this module is to resolve relevant variables for changing channels.
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

use crate::cc::error::ChangeChannelEntityError;
use crate::cc::reader::ChangeChannelReader;
use config::settings;
use db;
use db::pool::DbPool;
use entity::character::wrapper::Character;
use inc::helpers;
use session::model::Session;
use state::model::SharedState;

pub struct ChangeChannelStore {
    pub channel_id: u8,
    pub char: Character,
    pub map_wz: i32,
    pub octets: [u8; 4],
    pub port: i16,
    pub previous_channel_id: u8,
}

impl ChangeChannelStore {
    pub async fn store_change_channel(
        state: &SharedState,
        session: &Session,
        reader: &ChangeChannelReader,
    ) -> Result<Self, ChangeChannelEntityError> {
        let previous_channel_id: u8 = session.get_channel_id()?;
        let map_wz: i32 = session.get_map_wz()?;
        let char_id: i32 = session.get_char_id()?;
        let pool: DbPool = state.lock().await.db.clone();
        let char: Character =
            assembly::character::assemble::assemble_char_by_id(&pool, char_id).await?;
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = reader.channel_id;
        let port = {
            let state = state.lock().await;
            state
                .with_channel(world_id, channel_id, |channel| channel.model.port)
                .await?
        };
        let addr: String = settings::get_routing_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);
        Ok(Self {
            channel_id,
            char,
            map_wz,
            octets,
            port,
            previous_channel_id,
        })
    }
}
