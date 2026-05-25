/* list_worlds/handler.rs
 * The purpose of this module is to handle world listing.
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

use crate::list_worlds::error::ListWorldsError;
use crate::list_worlds::store::ListWorldsStore;
use crate::result::HandlerResult;
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use packet::build::list_worlds::builder;
use packet::model::Packet;
use state::model::SharedState;

pub struct ListWorldsHandler;

impl ListWorldsHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(&self, state: &SharedState) -> Result<HandlerResult, ListWorldsError> {
        let store: ListWorldsStore = ListWorldsStore::store_list_worlds(state).await?;
        let result: HandlerResult = self.build_list_worlds_result(&store).await?;
        Ok(result)
    }

    async fn build_list_worlds_result(
        &self,
        store: &ListWorldsStore,
    ) -> Result<HandlerResult, ListWorldsError> {
        let worlds = store.worlds_arc.read().await;
        let mut result: HandlerResult = HandlerResult::new();
        let packets: Vec<Packet> = builder::build_list_worlds_handler_servers_packets(&worlds)?;
        for packet in packets {
            result.add_action(Action::Session(SessionAction::Send {
                packet: packet.clone(),
                scope: SessionScope::Local,
            }));
        }
        let packet: Packet = Packet::new_empty()
            .build_list_worlds_handler_last_connected_world_packet()?
            .finish();
        result.add_action(Action::Session(SessionAction::Send {
            packet: packet.clone(),
            scope: SessionScope::Local,
        }));
        let packet: Packet = Packet::new_empty()
            .build_list_worlds_handler_recommended_worlds_packet(&worlds)?
            .finish();
        result.add_action(Action::Session(SessionAction::Send {
            packet: packet.clone(),
            scope: SessionScope::Local,
        }));
        Ok(result)
    }
}
