/* player_map_transfer/handler.rs
 * The purpose of this module is to handle player map transfers.
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

use crate::net::action::{Action, SessionAction};
use crate::net::packet::handler::player_map_transfer::error::PlayerMapTransferError;
use crate::net::packet::handler::player_map_transfer::reader::PlayerMapTransferReader;
use crate::net::packet::handler::player_map_transfer::store::PlayerMapTransferStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{MapScope, SessionScope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct PlayerMapTransferHandler;

impl PlayerMapTransferHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, PlayerMapTransferError> {
        let reader: PlayerMapTransferReader =
            PlayerMapTransferReader::read_player_map_transfer_packet(packet)?;
        let store: PlayerMapTransferStore =
            PlayerMapTransferStore::store_player_map_transfer(state, session, &reader).await?;
        let result: HandlerResult = self.build_player_map_transfer(&store)?;
        Ok(result)
    }

    fn build_player_map_transfer(
        &self,
        store: &PlayerMapTransferStore,
    ) -> Result<HandlerResult, PlayerMapTransferError> {
        let mut result = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_spawn_player_packet(&store.char)?
            .finish();
        result.add_action(Action::Session(SessionAction::Send {
            packet: packet.clone(),
            scope: SessionScope::Map(MapScope::SameChannelSameWorld),
        }));
        Ok(result)
    }
}
