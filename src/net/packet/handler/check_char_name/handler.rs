/* check_char_name/handler.rs
 * The purpose of this module is to handle character name checks.
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

use crate::net::action::Action;
use crate::net::packet::handler::check_char_name::error::CheckCharNameError;
use crate::net::packet::handler::check_char_name::reader::CheckCharNameReader;
use crate::net::packet::handler::check_char_name::store::CheckCharNameStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::Scope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct CheckCharNameHandler;

impl CheckCharNameHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, CheckCharNameError> {
        let reader: CheckCharNameReader = CheckCharNameReader::read_check_char_name_packet(packet)?;
        let store: CheckCharNameStore =
            CheckCharNameStore::store_check_char_name(state, session.clone(), reader)
                .await?;
        let result = self.build_check_char_name_result(store)?;
        Ok(result)
    }

    fn build_check_char_name_result(
        &self,
        store: CheckCharNameStore,
    ) -> Result<HandlerResult, CheckCharNameError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_check_char_name_packet(store.exists, store.ign.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        });
        Ok(result)
    }
}
