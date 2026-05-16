/* select_char_with_PIC/handler.rs
 * The purpose of this module is to handle PIC, character selection.
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
use crate::net::packet::handler::select_char_with_pic::error::SelectCharWithPicError;
use crate::net::packet::handler::select_char_with_pic::reader::SelectCharWithPicReader;
use crate::net::packet::handler::select_char_with_pic::store::SelectCharWithPicStore;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::Scope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct SelectCharWithPicHandler;

impl SelectCharWithPicHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, SelectCharWithPicError> {
        let reader: SelectCharWithPicReader =
            SelectCharWithPicReader::read_select_char_with_pic_packet(packet)?;
        let store: SelectCharWithPicStore =
            SelectCharWithPicStore::store_select_char_with_pic(state, session, &reader).await?;
        let result: HandlerResult = self.build_select_char_with_pic_result(&store)?;
        Ok(result)
    }

    fn build_select_char_with_pic_result(
        &self,
        store: &SelectCharWithPicStore,
    ) -> Result<HandlerResult, SelectCharWithPicError> {
        let mut result: HandlerResult = HandlerResult::new();
        if store.pic_status {
            result.add_action(Action::Set(SetAction::SetChar {
                char_id: store.char_id,
            }));
            let packet: Packet = Packet::new_empty()
                .build_select_char_packet(store.char_id, store.octets, store.port)?
                .finish();
            result.add_action(Action::Break {
                packet: packet.clone(),
                scope: Scope::Local,
            });
        } else {
            let packet: Packet = Packet::new_empty()
                .build_select_char_handler_failed_pic_packet()?
                .finish();
            result.add_action(Action::Send {
                packet: packet.clone(),
                scope: Scope::Local,
            });
        };
        Ok(result)
    }
}
