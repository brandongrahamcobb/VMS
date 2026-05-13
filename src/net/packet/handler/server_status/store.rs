/* server_status/store.rs
 * The purpose of this module is to resolve relevant variables for server status updates.
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

use crate::models::shroom::world::wrapper::World;
use crate::net::error::NetworkError;
use crate::net::packet::handler::server_status::reader::ServerStatusReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ServerStatusStore {
    pub worlds: Vec<World>,
    pub status: i16,
}

impl ServerStatusStore {
    pub async fn store_server_status(
        state: &SharedState,
        session: Session,
        reader: ServerStatusReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(session);
        std::hint::black_box(reader.clone());
        let worlds = {
            let state = state.lock().await;
            state.worlds.clone()
        };
        let status: i16 = if worlds.iter().any(|world| !world.channels.is_empty()) {
            0
        } else {
            2
        };
        Ok(Self { worlds, status })
    }
}
