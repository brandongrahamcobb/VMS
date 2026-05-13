/* list_chars/store.rs
 * The purpose of this module is to resolve relevant variables for character listing.
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
use crate::models::account::wrapper::Account;
use crate::models::character::wrapper::Character;
use crate::models::item::inventory::wrapper::InventoryItem;
use crate::models::shroom::channel::wrapper::Channel;
use crate::models::shroom::world::wrapper::World;
use crate::models::shroom::{channel, world};
use crate::models::{character, item};
use crate::net::error::NetworkError;
use crate::net::packet::handler::list_chars::reader::ListCharsReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ListCharsStore {
    pub channel: Channel,
    pub chars: Vec<Character>,
    pub char_slots: i16,
    pub world: World,
    pub pic_status: i16,
}

pub enum PicStatus {
    Disabled = 2,
    AlreadyRegistered = 1,
    NeedsToRegister = 0,
}

impl ListCharsStore {
    pub async fn store_list_chars(
        state: &SharedState,
        session: Session,
        reader: ListCharsReader,
    ) -> Result<Self, NetworkError> {
        let acc: Account = session.get_acc()?;
        let channel: Channel =
            channel::service::get_channel_by_id(state, reader.channel_id).await?;
        let world: World = world::service::get_world_by_id(state, reader.world_id).await?;
        let chars: Vec<Character> = acc.chars.clone();
        let char_slots: i16 = match character::query::getters::get_char_max_by_account_and_world_id(
            state,
            acc.model.get_id()?,
            world.model.id,
        )
        .await
        {
            Ok(char_max) => char_max,
            Err(_) => 8,
        };
        let mut pic_status: i16 = PicStatus::Disabled as i16;
        let use_pic = settings::get_pic_required()?;
        if let Some(_) = acc.model.clone().pic {
            if use_pic {
                pic_status = PicStatus::AlreadyRegistered as i16;
            }
        } else {
            pic_status = PicStatus::NeedsToRegister as i16;
        };
        Ok(Self {
            channel,
            chars: chars.clone(),
            char_slots,
            pic_status,
            world,
        })
    }
}
