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

use action::scope::SessionScope;
use action::model::{Action, SessionAction};
use crate::check_char_name::error::CheckCharNameError;
use crate::check_char_name::reader::CheckCharNameReader;
use crate::check_char_name::store::CheckCharNameStore;
use crate::result::HandlerResult;
use packet::model::Packet;
use db::pool::DbPool;

pub struct CheckCharNameHandler;

impl CheckCharNameHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        pool: &DbPool,
        packet: &Packet,
    ) -> Result<HandlerResult, CheckCharNameError> {
        let reader: CheckCharNameReader = CheckCharNameReader::read_check_char_name_packet(packet)?;
        let store: CheckCharNameStore =
            CheckCharNameStore::store_check_char_name(pool, &reader).await?;
        let result = self.build_check_char_name_result(&store)?;
        Ok(result)
    }

    fn build_check_char_name_result(
        &self,
        store: &CheckCharNameStore,
    ) -> Result<HandlerResult, CheckCharNameError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_check_char_name_packet(store.exists, store.ign.clone())?
            .finish();
        result.add_action(Action::Session(SessionAction::Send {
            packet: packet.clone(),
            scope: SessionScope::Local,
        }));
        Ok(result)
    }
}
