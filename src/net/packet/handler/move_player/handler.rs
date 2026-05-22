/* move_player/handler.rs
 * The purpose of this module is to handle player movement.
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

use crate::net::action::{Action, BroadcastAction};
use crate::net::packet::handler::move_player::error::MovePlayerError;
use crate::net::packet::handler::move_player::reader::MovePlayerReader;
use crate::net::packet::handler::move_player::store::MovePlayerStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::BroadcastScope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct MovePlayerHandler;

impl MovePlayerHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, MovePlayerError> {
        if session.transitioning {
            return Ok(HandlerResult::new());
        }
        let reader: MovePlayerReader = MovePlayerReader::read_move_player_packet(packet)?;
        let store: MovePlayerStore =
            MovePlayerStore::store_move_player(state, session, &reader).await?;
        let result: HandlerResult = self.build_move_player_result(&store)?;
        Ok(result)
    }

    fn build_move_player_result(
        &self,
        store: &MovePlayerStore,
    ) -> Result<HandlerResult, MovePlayerError> {
        let mut result = HandlerResult::new();
        if !store.too_short && !store.empty {
            let packet: Packet = Packet::new_empty()
                .build_player_move_packet(store.char_id, store.movement_bytes.clone())?
                .finish();
            result.add_action(Action::Broadcast(BroadcastAction::Send {
                packet: packet.clone(),
                scope: BroadcastScope::Map {
                    world_id: store.world_id,
                    channel_id: store.channel_id,
                    map_wz: store.map_wz,
                },
            }));
        }
        Ok(result)
    }
}
