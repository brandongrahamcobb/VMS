use crate::config::settings;
use crate::models::account::model::Account;
use crate::models::channel::model::ChannelModel;
use crate::models::character::model::Character;
use crate::models::world::model::WorldModel;
use crate::models::{channel, world};
use crate::net::error::NetworkError;
use crate::net::packet::handler::list_chars::reader::ListCharsReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ListCharsStore {
    pub channel: ChannelModel,
    pub chars: Vec<Character>,
    pub char_max: i16,
    pub pic_status: i8,
    pub world: WorldModel,
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
        let acc: Account = session.acc.clone();
        let chars: Vec<Character> = acc.chars.clone();
        let world: WorldModel = world::service::get_world_by_id(reader.world_id)?;
        let channel: ChannelModel =
            channel::service::get_channel_by_ids(reader.channel_id, world.id)?;
        let char_max =
            world::query::get_character_max_by_account_and_world_id(state, acc.id, world.id)
                .await?;
        let mut pic_status: i8 = PicStatus::Disabled as i8;
        let use_pic = settings::get_pic_required()?;
        pic_status = if use_pic {
            PicStatus::AlreadyRegistered as i8
        } else {
            PicStatus::NeedsToRegister as i8
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
