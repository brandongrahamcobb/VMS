/* shared.rs
 * The purpose of this module is to provide the shared relay.
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
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::RuntimeError;
use crate::runtime::relay::execute;
use crate::runtime::session::error::SessionError;
use crate::runtime::state::SharedState;
use core::ops::ControlFlow;

#[allow(async_fn_in_trait)]
pub trait RuntimeRelay: Sized {
    async fn new(session_id: i32) -> Result<Self, RuntimeError>;

    fn session_id(&self) -> i32;

    async fn handle_packet(
        &mut self,
        state: &SharedState,
        packet: &Packet,
    ) -> Result<HandlerResult, RuntimeError>;

    async fn execute(
        &mut self,
        state: &SharedState,
        result: HandlerResult,
    ) -> Result<ControlFlow<Packet>, RuntimeError> {
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
                Action::Break { packet, scope } => {
                    return execute::manager::end(state, &session, &packet, &scope).await;
                }
                Action::Send { packet, scope } => {
                    execute::manager::send(state, &session, &packet, &scope).await?
                }
                Action::Set(set_action) => match set_action {
                    SetAction::SetMap { map, scope } => {
                        execute::manager::set_map(state, &session, &map, &scope).await?
                    }
                    SetAction::SetChannel { channel, scope } => {
                        execute::manager::set_channel(state, &session, &channel, &scope).await?
                    }
                    SetAction::SetWorld { world, scope } => {
                        execute::manager::set_world(state, &session, &world, &scope).await?
                    }
                    SetAction::SetAccount { acc } => {
                        execute::manager::set_acc(state, &session, &acc).await?
                    }
                    SetAction::SetChar { char } => {
                        execute::manager::set_char(state, &session, &char).await?
                    }
                },
            }
        }
        return Ok(ControlFlow::Continue(()));
    }
}
