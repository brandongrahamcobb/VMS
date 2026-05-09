use crate::config::settings;
use crate::models::account::model::Account;
use crate::models::character;
use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::models::shroom::world::model::World;
use crate::models::shroom::{channel, world};
use crate::net::error::NetworkError;
use crate::net::packet::handler::list_chars::reader::ListCharsReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ListCharsStore {
    pub channel: Channel,
    pub chars: Vec<Character>,
    pub char_max: i16,
    pub world: World,
    pub pic_status: i16,
}

pub enum PicStatus {
    Disabled = 0,
    AlreadyRegistered = 1,
    NeedsToRegister = 2,
}

impl ListCharsStore {
    pub async fn store_list_chars(
        state: &SharedState,
        session: Session,
        reader: ListCharsReader,
    ) -> Result<Self, NetworkError> {
        let acc: Account = session.get_acc()?;
        let chars: Vec<Character> = acc.chars.clone();
        let world: World = world::service::get_world_by_id(state, reader.world_id).await?;
        let channel: Channel =
            channel::service::get_channel_by_id(state, reader.channel_id).await?;
        let char_max: i16 =
            match character::query::getters::get_character_max_by_account_and_world_id(
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
            chars,
            char_max,
            pic_status,
            world,
        })
    }
}
