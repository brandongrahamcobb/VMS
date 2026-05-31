/* player.rs
 * The purpose of this module is to provide the player relay.
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

use crate::relay::model::PlayerRelay;
use crate::relay::types::error::RelayTypeError;
use crate::relay::types::shared::RuntimeRelay;
use action::event::TickEvent;
use net::packet::cc::handler::ChangeChannelHandler;
use net::packet::change_keymap::handler::ChangeKeymapHandler;
use net::packet::change_map::handler::ChangeMapHandler;
use net::packet::chat_text::handler::ChatTextHandler;
use net::packet::close_attack::handler::CloseAttackHandler;
use net::packet::enter_cash_shop::handler::EnterCashShopHandler;
use net::packet::error::PacketHandlerError;
use net::packet::mob_ai::handler::MobAiHandler;
use net::packet::move_player::handler::MovePlayerHandler;
use net::packet::party_search::handler::PartySearchHandler;
use net::packet::pickup_item::handler::PickupItemHandler;
use net::packet::player_logged_in::handler::PlayerLoggedInHandler;
use net::packet::player_map_transfer::handler::PlayerMapTransferHandler;
use net::packet::result::HandlerResult;
use net::packet::take_damage::handler::TakeDamageHandler;
use op::recv::RecvOpcode;
use net::packet::model::Packet;
use net::packet::prelude::*;
use session::error::SessionError;
use state::model::SharedState;
use tokio::sync::broadcast;
use tracing::debug;

impl RuntimeRelay for PlayerRelay {
    async fn new(session_id: i32) -> Result<Self, RelayTypeError> {
        Ok(Self {
            session_id,
            tick_rx: None,
        })
    }

    fn tick_rx(&mut self) -> Option<&mut broadcast::Receiver<TickEvent>> {
        self.tick_rx.as_mut()
    }

    fn set_tick_rx(&mut self, rx: broadcast::Receiver<TickEvent>) {
        self.tick_rx = Some(rx);
    }

    fn session_id(&self) -> i32 {
        self.session_id
    }

    async fn handle_packet(
        &mut self,
        state: &SharedState,
        packet: &Packet,
    ) -> Result<HandlerResult, RelayTypeError> {
        let session = {
            let state = state.lock().await;
            state
                .sessions
                .get(self.session_id())
                .ok_or(SessionError::NotFound(self.session_id()))?
        };
}
