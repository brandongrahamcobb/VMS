/* change_map/handler.rs
 * The purpose of this module is to handle map changes.
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

use crate::net::action::{Action, SetAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::change_map::reader::ChangeMapReader;
use crate::net::packet::handler::change_map::store::ChangeMapStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{MapScope, Scope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ChangeMapHandler;

impl ChangeMapHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: ChangeMapReader = ChangeMapReader::read_change_map_packet(packet)?;
        let store: ChangeMapStore =
            ChangeMapStore::store_change_map(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_change_map(store.clone())?;
        Ok(result)
    }

    fn build_change_map(&self, store: ChangeMapStore) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_packet(store.char.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        })?;
        result.add_action(Action::Set(SetAction::SetChar {
            char: store.char.clone(),
        }))?;
        let packet: Packet = Packet::new_empty()
            .build_set_field_change_map_packet(
                store.channel.clone(),
                store.map.clone(),
                store.portal.model.pid,
            )?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        result.add_action(Action::Set(SetAction::SetMap {
            map: store.map.clone(),
            scope: Scope::Local,
        }))?;
        let packet: Packet = Packet::new_empty()
            .build_spawn_player_packet(store.char.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        })?;
        for player in store.after_players {
            let packet: Packet = Packet::new_empty()
                .build_spawn_player_packet(player.clone())?
                .finish();
            result.add_action(Action::Send {
                packet: packet.clone(),
                scope: Scope::Local,
            })?;
        }
        Ok(result)
    }
}
