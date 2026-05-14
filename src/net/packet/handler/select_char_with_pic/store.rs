/* select_char_with_pic/store.rs
 * The purpose of this module is to resolve relevant variables for PIC character selection.
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

use crate::config::settings;
use crate::inc::helpers;
use crate::models::account::wrapper::Account;
use crate::models::character;
use crate::models::character::wrapper::Character;
use crate::models::channel::wrapper::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::select_char_with_pic::reader::SelectCharWithPicReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct SelectCharWithPicStore {
    pub char: Character,
    pub channel: Channel,
    pub octets: [u8; 4],
    pub pic_status: bool,
}

impl SelectCharWithPicStore {
    pub async fn store_select_char_with_pic(
        state: &SharedState,
        session: Session,
        reader: SelectCharWithPicReader,
    ) -> Result<Self, NetworkError> {
        let acc: Account = session.get_acc()?;
        let channel: Channel = session.get_active_channel(state).await?;
        let char: Character =
            character::service::get_char_by_id(state, reader.char_id).await?;
        let acc_pic = acc.model.get_pic()?;
        let mut pic_status = false;
        if acc_pic == reader.pic {
            pic_status = true;
        }
        let addr: String = settings::get_routing_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr.clone());
        Ok(Self {
            channel,
            char,
            pic_status,
            octets,
        })
    }
}
