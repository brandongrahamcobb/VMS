ment_set::model::{CashEquipmentSet, RegularEquipmentSet};
use crate::config::settings;
use crate::models::account::model::Account;
use crate::models::channel::model::Channel;
use crate::models::{account, channel, character, world};
use crate::net::packet::handler::list_chars::reader::ListCharsRead;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct ListCharsStore {
    pub acc: Account,
    pub chars: Vec<Character>,
    pub channel: Channel,
    pub char_max: u8,
    pub pic_status: u8,
}

pub enum PicStatus {
    Disabled = 0,
    AlreadyRegistered = 1,
    NeedsToRegister = 2,
}

impl ListCharsStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_list_chars(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &ListCharsRead
    ) -> Result<Self, NetworkError> {
        let world = world::service::get_world_by_id(&reader.world_id)?;
        let channel = channel::service::get_channel_by_ids(&reader.channel_id, &reader.world_id)?;
        let chars = character::query::get_characters_by_account_id_and_world_id(
            state,
            &session.acc.id,
            &reader.world_id,
        )
        .await?;
        let default_char_max = settings::get_char_max()?;
        let char_max =
            world::query::get_character_max_by_account_and_world_id(state, &session.acc.id, &reader.world_id)
                .await
                .unwrap_or(default_char_max as i16);
        let mut pic_status: u8 = PicStatus::Disabled as u8;
        let use_pic = settings::get_pic_required()?;
        if let Some(_pic) = *session.acc.pic {
            pic_status = if use_pic { PicStatus::AlreadyRegistered as u8 } else { PicStatus::NeedsToRegister as u8 }; 
        }
        Ok(Self {
            acc: session.acc.clone(),
            chars: chars.clone(),
            channel: channel.clone(),
            char_max: char_max.clone(),
            pic_status: pic_status.clone(),
        })
    }
}
