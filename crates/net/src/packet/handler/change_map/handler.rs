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

use crate::action::model::{Action, SessionAction, SetAction};
use crate::packet::handler::change_map::error::ChangeMapEntityError;
use crate::packet::handler::change_map::reader::ChangeMapReader;
use crate::packet::handler::change_map::store::ChangeMapStore;
use crate::packet::handler::result::HandlerResult;
use packet::model::Packet;
use crate::action::scope::{MapScope, SessionScope};
use session::model::Session;
use state::model::SharedState;

pub struct ChangeMapHandler;

impl ChangeMapHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, ChangeMapEntityError> {
        if session.transitioning {
            return Ok(HandlerResult::new());
        }
        let reader: ChangeMapReader = ChangeMapReader::read_change_map_packet(packet)?;
        let store: ChangeMapStore =
            ChangeMapStore::store_change_map(state, session, &reader).await?;
        let result: HandlerResult = self.build_change_map(&store)?;
        Ok(result)
    }

    fn build_change_map(&self, store: &ChangeMapStore) -> Result<HandlerResult, ChangeMapEntityError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_packet(&store.char)?
            .finish();
        result.add_action(Action::Session(SessionAction::Send {
            packet: packet.clone(),
            scope: SessionScope::Map(MapScope::SameChannelSameWorld),
        }));
        let packet: Packet = Packet::new_empty()
            .build_set_field_change_map_packet(store.channel_id, store.after_map_wz, store.pid)?
            .finish();
        result.add_action(Action::Session(SessionAction::Send {
            packet: packet.clone(),
            scope: SessionScope::Local,
        }));
        result.add_action(Action::Session(SessionAction::Set(SetAction::SetMap {
            map_wz: store.after_map_wz,
            scope: SessionScope::Local,
        })));
        Ok(result)
    }
}
