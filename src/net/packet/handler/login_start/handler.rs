/* login_start/handler.rs
 * The purpose of this module is to handle a login start.
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

use crate::net::packet::handler::login_start::error::LoginStartError;
use crate::net::packet::handler::login_start::reader::LoginStartReader;
use crate::net::packet::handler::login_start::store::LoginStartStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct LoginStartHandler;

impl LoginStartHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, LoginStartError> {
        let reader: LoginStartReader = LoginStartReader::read_login_start_packet(packet)?;
        let store: LoginStartStore =
            LoginStartStore::store_login_start(state, session.clone(), reader.clone()).await?;
        let result = self.build_login_start_result(store.clone())?;
        Ok(result)
    }

    fn build_login_start_result(
        &self,
        store: LoginStartStore,
    ) -> Result<HandlerResult, LoginStartError> {
        // not implemented
        std::hint::black_box(store);
        let result: HandlerResult = HandlerResult::new();
        Ok(result)
    }
}
