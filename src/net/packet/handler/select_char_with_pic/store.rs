use crate::models::account::model::Account;
use crate::models::character::model::Character;
use crate::models::world::model::World;
use crate::models::{channel, character, world};
use crate::net::error::NetworkError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use core::net::Ipv4Addr;

pub struct SelectCharWithPicStore {
    pub char: Character,
    pub ip: Ipv4Addr,
    pub world: World,
    pub channel: Channel,
    pub pic_status: bool,
}

impl SelectCharWithPicStore {
    pub fn new() -> Self {
        Self
    }

    pub fn store_select_char_with_pic(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &SelectCharWithPicReader,
    ) -> Result<Self, NetworkError> {
        let char: Character = character::query::get_character_by_id(state, &reader.char_id)?;
        let acc: Account = session.acc.ok_or(SessionError::NoAccount(session.id))?;
        let acc_pic = acc.pic.ok_or(AccountError::MissingField(acc.id))?;
        let mut pic_status = false;
        if acc_pic.as_ref() == &reader.pic {
            pic_status = true;
        }
        let addr = settings::get_address()?;
        let octets = helpers::convert_to_ip_array(&addr);
        let ip = Ipv4Addr::new(octets)?;
        let world_id = session.world_id.ok_or(SessionError::NoWorld(session.id))?;
        let world: World = world::service::get_world_by_id(&world_id)?;
        let channel_id = session
            .channel_id
            .ok_or(SessionError::NoChannel(session.id))?;
        let channel: Channel = channel::service::get_channel_by_ids(&channel_id, &world_id)?;
        Ok(Self {
            char: char.clone(),
            ip: ip.clone(),
            world: world.clone(),
            channel: channel.clone(),
            pic_status: pic_status.clone(),
        })
    }
}
