/* list_worlds/store.rs
 * The purpose of this module is to resolve relevant variables for world listing.
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

use crate::models::world::wrapper::World;
use crate::net::packet::handler::list_worlds::error::ListWorldsError;
use crate::net::packet::handler::list_worlds::reader::ListWorldsReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ListWorldsStore {
    pub worlds_arc: Arc<RwLock<HashMap<i16, World>>>,
}

impl ListWorldsStore {
    pub async fn store_list_worlds(
        state: &SharedState,
        session: &Session,
        reader: &ListWorldsReader,
    ) -> Result<Self, ListWorldsError> {
        std::hint::black_box(session);
        std::hint::black_box(reader);
        let worlds_arc = {
            let state = state.lock().await;
            state.worlds.clone()
        };
        Ok(Self { worlds_arc })
    }
}
