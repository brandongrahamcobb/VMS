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
use net::packet::handler::cc::handler::ChangeChannelHandler;
use net::packet::handler::change_keymap::handler::ChangeKeymapHandler;
use net::packet::handler::change_map::handler::ChangeMapHandler;
use net::packet::handler::chat_text::handler::ChatTextHandler;
use net::packet::handler::close_attack::handler::CloseAttackHandler;
use net::packet::handler::enter_cash_shop::handler::EnterCashShopHandler;
use net::packet::handler::error::PacketHandlerError;
use net::packet::handler::mob_ai::handler::MobAiHandler;
use net::packet::handler::move_player::handler::MovePlayerHandler;
use net::packet::handler::party_search::handler::PartySearchHandler;
use net::packet::handler::pickup_item::handler::PickupItemHandler;
use net::packet::handler::player_logged_in::handler::PlayerLoggedInHandler;
use net::packet::handler::player_map_transfer::handler::PlayerMapTransferHandler;
use net::packet::handler::result::HandlerResult;
use net::packet::handler::take_damage::handler::TakeDamageHandler;
use op::recv::RecvOpcode;
use packet::model::Packet;
use packet::prelude::*;
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
        let op = packet.opcode();
        let en = RecvOpcode::from_i16(op).ok_or(RelayTypeError::UnsupportedOpcodeError(
            op,
            String::from("not expected in channel"),
        ));
        debug!(
            "Received opcode in channel: {} (0x{:02X}) ({:?})",
            op, op, en
        );
        match op {
            x if x == RecvOpcode::PlayerLoggedIn as i16 => {
                let handler = PlayerLoggedInHandler::new();
                let pool = { state.lock().await.db.clone() };
                Ok(handler
                    .handle(&pool, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::ChangeChannel as i16 => {
                let handler = ChangeChannelHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::PartySearch as i16 => {
                let handler = PartySearchHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::PlayerMapTransfer as i16 => {
                let handler = PlayerMapTransferHandler::new();
                Ok(handler
                    .handle(state, &session)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::PlayerMove as i16 => {
                let handler = MovePlayerHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::EnterCashShop as i16 => {
                let handler = EnterCashShopHandler::new();
                let pool = { state.lock().await.db.clone() };
                Ok(handler
                    .handle(&pool, &session)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::ChangeKeymap as i16 => {
                let handler = ChangeKeymapHandler::new();
                let pool = { state.lock().await.db.clone() };
                Ok(handler
                    .handle(&pool, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::CloseAttack as i16 => {
                let handler = CloseAttackHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::AllChat as i16 => {
                let handler = ChatTextHandler::new();
                let pool = { state.lock().await.db.clone() };
                Ok(handler
                    .handle(&pool, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::ChangeMap as i16 => {
                let handler = ChangeMapHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::MobMoved as i16 => {
                let handler = MobAiHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::TakeDamage as i16 => {
                let handler = TakeDamageHandler::new();
                let pool = { state.lock().await.db.clone() };
                Ok(handler
                    .handle(&pool, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::PickupItem as i16 => {
                let handler = PickupItemHandler::new();
                let pool = { state.lock().await.db.clone() };
                Ok(handler
                    .handle(&pool, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            _ => Err(RelayTypeError::UnsupportedOpcodeError(
                op,
                String::from("expected in channel"),
            )),
        }
    }
}
