/* login.rs
 * The purpose of this module is to provide the login relay.
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

use crate::net::packet::handler::check_char_name::handler::CheckCharNameHandler;
use crate::net::packet::handler::create_char::handler::CreateCharHandler;
use crate::net::packet::handler::credentials::handler::CredentialsHandler;
use crate::net::packet::handler::delete_char::handler::DeleteCharHandler;
use crate::net::packet::handler::error::PacketHandlerError;
use crate::net::packet::handler::list_chars::handler::ListCharsHandler;
use crate::net::packet::handler::list_worlds::handler::ListWorldsHandler;
use crate::net::packet::handler::login_start::handler::LoginStartHandler;
use crate::net::packet::handler::register_pic::handler::RegisterPicHandler;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::select_char::handler::SelectCharHandler;
use crate::net::packet::handler::select_char_with_pic::handler::SelectCharWithPicHandler;
use crate::net::packet::handler::server_status::handler::ServerStatusHandler;
use crate::net::packet::handler::tos::handler::TosHandler;
use crate::net::packet::model::Packet;
use crate::op::recv::RecvOpcode;
use crate::prelude::*;
use crate::runtime::relay::model::LoginRelay;
use crate::runtime::relay::types::error::RelayTypeError;
use crate::runtime::relay::types::shared::RuntimeRelay;
use crate::runtime::session::error::SessionError;
use crate::runtime::state::SharedState;
use tokio::sync::broadcast;
use tracing::debug;

impl RuntimeRelay for LoginRelay {
    async fn new(session_id: i32) -> Result<Self, RelayTypeError> {
        Ok(Self { session_id })
    }

    fn tick_rx(&mut self) -> Option<&mut broadcast::Receiver<HandlerResult>> {
        None
    }

    fn set_tick_rx(&mut self, _rx: broadcast::Receiver<HandlerResult>) {}

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
            String::from("not expected during authentication"),
        ));
        debug!(
            "Received opcode in login: {} (0x{:02X}) ({:?}),",
            op, op, en
        );
        match op {
            x if x == RecvOpcode::RequestLogin as i16 => {
                let handler = CredentialsHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::LoginStarted as i16 => {
                let handler = LoginStartHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::AcceptTOS as i16 => {
                let handler = TosHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::ServerListRequest as i16 => {
                let handler = ListWorldsHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::ServerStatusRequest as i16 => {
                let handler = ServerStatusHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::CharListRequest as i16 => {
                let handler = ListCharsHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::CreateChar as i16 => {
                let handler = CreateCharHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::CheckCharName as i16 => {
                let handler = CheckCharNameHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::DeleteChar as i16 => {
                let handler = DeleteCharHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::CharSelect as i16 => {
                let handler = SelectCharHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::RegisterPic as i16 => {
                let handler = RegisterPicHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::CharSelectWithPic as i16 => {
                let handler = SelectCharWithPicHandler::new();
                Ok(handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            _ => Err(RelayTypeError::UnsupportedOpcodeError(
                op,
                String::from("expected after authentication"),
            )),
        }
    }
}
