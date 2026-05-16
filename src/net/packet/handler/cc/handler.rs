/* cc/handler.rs
 * The purpose of this module is to handle channel changes.
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
use crate::net::packet::handler::cc::error::ChangeChannelError;
use crate::net::packet::handler::cc::reader::ChangeChannelReader;
use crate::net::packet::handler::cc::store::ChangeChannelStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{MapScope, Scope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ChangeChannelHandler;

impl ChangeChannelHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, ChangeChannelError> {
        let reader: ChangeChannelReader = ChangeChannelReader::read_change_channel_packet(packet)?;
        let store: ChangeChannelStore =
            ChangeChannelStore::store_change_channel(state, session, reader).await?;
        let result: HandlerResult = self.build_change_channel_result(store).await?;
        Ok(result)
    }

    async fn build_change_channel_result(
        &self,
        store: ChangeChannelStore,
    ) -> Result<HandlerResult, ChangeChannelError> {
        let mut result: HandlerResult = HandlerResult::new();
        result.add_action(Action::Set(SetAction::SetChannel {
            channel_id: store.channel_id,
            scope: Scope::Local,
        }));
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_packet(&store.char)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        });
        let packet: Packet = Packet::new_empty()
            .build_spawn_player_packet(&store.char)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        });
        let packet: Packet = Packet::new_empty()
            .build_channel_change_packet(store.octets.clone(), store.port)?
            .finish();
        result.add_action(Action::Break {
            packet: packet.clone(),
            scope: Scope::Local,
        });
        result.add_action(Action::Retrieve);
        Ok(result)
    }
}
