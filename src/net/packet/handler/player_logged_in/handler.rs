/* player_logged_in/handler.rs
 * The purpose of this module is to handle player login.
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

use crate::net::action::{Action, SessionAction, SetAction};
use crate::net::packet::handler::player_logged_in::error::PlayerLoggedInError;
use crate::net::packet::handler::player_logged_in::reader::PlayerLoggedInReader;
use crate::net::packet::handler::player_logged_in::store::PlayerLoggedInStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::SessionScope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct PlayerLoggedInHandler;

impl PlayerLoggedInHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, PlayerLoggedInError> {
        let reader: PlayerLoggedInReader =
            PlayerLoggedInReader::read_player_logged_in_packet(packet)?;
        let store: PlayerLoggedInStore =
            PlayerLoggedInStore::store_player_logged_in(state, session, &reader).await?;
        let result: HandlerResult = self.build_player_logged_in_result(&store)?;
        Ok(result)
    }

    fn build_player_logged_in_result(
        &self,
        store: &PlayerLoggedInStore,
    ) -> Result<HandlerResult, PlayerLoggedInError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_player_logged_in_keymap_packet(&store.binds)?
            .finish();
        result.add_action(Action::Session(SessionAction::Send {
            packet: packet.clone(),
            scope: SessionScope::Local,
        }));
        let packet: Packet = Packet::new_empty()
            .build_set_field_packet(&store.char, store.channel_id, store.map_wz)?
            .finish();
        result.add_action(Action::Session(SessionAction::Send {
            packet: packet.clone(),
            scope: SessionScope::Local,
        }));
        result.add_action(Action::Session(SessionAction::Set(SetAction::SetMap {
            map_wz: store.map_wz,
            scope: SessionScope::Local,
        })));
        Ok(result)
    }
}
