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

use crate::relay::model::LoginRelay;
use crate::relay::types::error::RelayTypeError;
use crate::relay::types::shared::RuntimeRelay;
use action::event::TickEvent;
use net::check_char_name::handler::CheckCharNameHandler;
use net::create_char::handler::CreateCharHandler;
use net::credentials::handler::CredentialsHandler;
use net::delete_char::handler::DeleteCharHandler;
use net::error::PacketHandlerError;
use net::list_chars::handler::ListCharsHandler;
use net::list_worlds::handler::ListWorldsHandler;
use net::login_start::handler::LoginStartHandler;
use net::register_pic::handler::RegisterPicHandler;
use net::result::HandlerResult;
use net::select_char::handler::SelectCharHandler;
use net::select_char_with_pic::handler::SelectCharWithPicHandler;
use net::server_status::handler::ServerStatusHandler;
use net::tos::handler::TosHandler;
use op::recv::RecvOpcode;
use packet::model::Packet;
use packet::prelude::*;
use session::error::SessionError;
use state::model::SharedState;
use tokio::sync::broadcast;
use tracing::debug;

impl RuntimeRelay for LoginRelay {
    async fn new(session_id: i32) -> Result<Self, RelayTypeError> {
        Ok(Self { session_id })
    }

    fn tick_rx(&mut self) -> Option<&mut broadcast::Receiver<TickEvent>> {
        None
    }

    fn set_tick_rx(&mut self, _rx: broadcast::Receiver<TickEvent>) {}

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
                Ok(handler.handle().await.map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::AcceptTOS as i16 => {
                let handler = TosHandler::new();
                let pool = { state.lock().await.db.clone() };
                Ok(handler
                    .handle(&pool, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::ServerListRequest as i16 => {
                let handler = ListWorldsHandler::new();
                Ok(handler
                    .handle(state)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::ServerStatusRequest as i16 => {
                let handler = ServerStatusHandler::new();
                Ok(handler
                    .handle(state)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::CharListRequest as i16 => {
                let handler = ListCharsHandler::new();
                let pool = { state.lock().await.db.clone() };
                Ok(handler
                    .handle(&pool, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::CreateChar as i16 => {
                let handler = CreateCharHandler::new();
                let pool = { state.lock().await.db.clone() };
                Ok(handler
                    .handle(&pool, &session, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::CheckCharName as i16 => {
                let handler = CheckCharNameHandler::new();
                let pool = { state.lock().await.db.clone() };
                Ok(handler
                    .handle(&pool, packet)
                    .await
                    .map_err(PacketHandlerError::from)?)
            }
            x if x == RecvOpcode::DeleteChar as i16 => {
                let handler = DeleteCharHandler::new();
                let pool = { state.lock().await.db.clone() };
                Ok(handler
                    .handle(&pool, &session, packet)
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
