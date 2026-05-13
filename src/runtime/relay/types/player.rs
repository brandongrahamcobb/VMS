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

use crate::net::packet::handler::cc::handler::ChangeChannelHandler;
use crate::net::packet::handler::change_keymap::handler::ChangeKeymapHandler;
use crate::net::packet::handler::change_map::handler::ChangeMapHandler;
use crate::net::packet::handler::chat_text::handler::ChatTextHandler;
use crate::net::packet::handler::close_attack::handler::CloseAttackHandler;
use crate::net::packet::handler::enter_cash_shop::handler::EnterCashShopHandler;
use crate::net::packet::handler::move_player::handler::MovePlayerHandler;
use crate::net::packet::handler::party_search::handler::PartySearchHandler;
use crate::net::packet::handler::player_logged_in::handler::PlayerLoggedInHandler;
use crate::net::packet::handler::player_map_transfer::handler::PlayerMapTransferHandler;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::op::recv::RecvOpcode;
use crate::prelude::*;
use crate::runtime::error::RuntimeError;
use crate::runtime::relay::model::PlayerRelay;
use crate::runtime::relay::types::shared::RuntimeRelay;
use crate::runtime::session::error::SessionError;
use crate::runtime::state::SharedState;
use tracing::debug;

impl RuntimeRelay for PlayerRelay {
    async fn new(session_id: i32) -> Result<Self, RuntimeError> {
        Ok(Self { session_id })
    }

    fn session_id(&self) -> i32 {
        self.session_id
    }

    async fn handle_packet(
        &mut self,
        state: &SharedState,
        packet: &Packet,
    ) -> Result<HandlerResult, RuntimeError> {
        let session = {
            let state = state.lock().await;
            state
                .sessions
                .get(self.session_id())
                .ok_or(SessionError::NotFound(self.session_id()))?
        };
        let op = packet.opcode();
        let en = RecvOpcode::from_i16(op).ok_or(RuntimeError::UnsupportedOpcodeError(
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
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::ChangeChannel as i16 => {
                let handler = ChangeChannelHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::PartySearch as i16 => {
                let handler = PartySearchHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::PlayerMapTransfer as i16 => {
                let handler = PlayerMapTransferHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::PlayerMove as i16 => {
                let handler = MovePlayerHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::EnterCashShop as i16 => {
                let handler = EnterCashShopHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::ChangeKeymap as i16 => {
                let handler = ChangeKeymapHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::CloseAttack as i16 => {
                let handler = CloseAttackHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::AllChat as i16 => {
                let handler = ChatTextHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::ChangeMap as i16 => {
                let handler = ChangeMapHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            _ => Err(RuntimeError::UnsupportedOpcodeError(
                op,
                String::from("expected in channel"),
            )),
        }
    }
}
