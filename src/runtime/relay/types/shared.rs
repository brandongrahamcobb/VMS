/* shared.rs The purpose of this module is to provide the shared relay.
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

use tokio::sync::broadcast;

use crate::net::action::{Action, BroadcastAction, SessionAction, SetAction};
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::execute;
use crate::runtime::relay::types::error::RelayTypeError;
use crate::runtime::session::error::SessionError;
use crate::runtime::state::SharedState;
use core::ops::ControlFlow;

#[allow(async_fn_in_trait)]
pub trait RuntimeRelay: Sized {
    fn tick_rx(&mut self) -> Option<&mut broadcast::Receiver<HandlerResult>>;

    fn set_tick_rx(&mut self, rx: broadcast::Receiver<HandlerResult>);

    async fn new(session_id: i32) -> Result<Self, RelayTypeError>;

    fn session_id(&self) -> i32;

    async fn handle_packet(
        &mut self,
        state: &SharedState,
        packet: &Packet,
    ) -> Result<HandlerResult, RelayTypeError>;

    async fn execute_with_session(
        &mut self,
        state: &SharedState,
        result: HandlerResult,
    ) -> Result<ControlFlow<Packet>, RelayTypeError> {
        let model = &result.model;
        for action in model {
            let session = {
                let state = state.lock().await;
                state
                    .sessions
                    .get(self.session_id())
                    .ok_or(SessionError::NotFound(self.session_id()))?
            };
            match action {
                Action::Broadcast(action) => match action {
                    BroadcastAction::Send { packet, scope } => {
                        if session.transitioning {
                            return Ok(ControlFlow::Continue(()));
                        }
                        execute::broadcast_execute::broadcast(state, &packet, &scope).await?;
                    }
                },
                Action::Session(action) => match action {
                    SessionAction::Break { packet, scope } => {
                        return execute::session_execute::end(state, &session, &packet, &scope)
                            .await
                            .map_err(RelayTypeError::from);
                    }
                    SessionAction::Send { packet, scope } => {
                        let _ = execute::session_execute::send(state, &session, &packet, &scope)
                            .await?;
                    }
                    SessionAction::Retrieve => {
                        execute::session_execute::retrieve(state, &session).await?
                    }
                    SessionAction::Set(set_action) => match set_action {
                        SetAction::SetMap { map_wz, scope } => {
                            if session.get_map_wz().is_ok() {
                                execute::session_execute::exit_map(state, &session).await?;
                            }
                            let tick_rx = execute::session_execute::enter_map(
                                state, &session, &scope, *map_wz,
                            )
                            .await?;
                            self.set_tick_rx(tick_rx);
                        }
                        SetAction::SetChannel { channel_id, scope } => {
                            execute::session_execute::set_channel(
                                state,
                                &session,
                                &scope,
                                *channel_id,
                            )
                            .await?
                        }
                        SetAction::SetWorld { world_id, scope } => {
                            execute::session_execute::set_world(state, &session, &scope, *world_id)
                                .await?
                        }
                        SetAction::SetAccount { acc_id } => {
                            execute::session_execute::set_acc(state, &session, *acc_id).await?
                        }
                        SetAction::SetChar { char_id } => {
                            execute::session_execute::set_char(state, &session, *char_id).await?
                        }
                    },
                },
            }
        }
        return Ok(ControlFlow::Continue(()));
    }

    async fn execute_via_tick(
        &self,
        state: &SharedState,
        result: HandlerResult,
    ) -> Result<ControlFlow<Packet>, RelayTypeError> {
        let model = &result.model;
        for action in model {
            match action {
                Action::Broadcast(action) => match action {
                    BroadcastAction::Send { packet, scope } => {
                        execute::broadcast_execute::broadcast(state, &packet, &scope).await?
                    }
                },
                Action::Session(_) => {}
            }
        }
        return Ok(ControlFlow::Continue(()));
    }
}
