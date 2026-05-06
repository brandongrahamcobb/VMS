use crate::config::settings;
use crate::models::channel::model::Channel;
use crate::models::character::model::Character;
use crate::models::map::model::Map;
use crate::models::world::model::World;
use crate::models::{channel, world};
use crate::net::error::NetworkError;
use crate::net::packet::handler::register_pic::reader::RegisterPicReader;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use core::net::Ipv4Addr;

pub struct RegisterPicStore {
    pub char: Character,
    pub world: World,
    pub channel: Channel,
    pub ip: Ipv4Addr,
}

impl RegisterPicStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_register_pic(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &RegisterPicReader,
    ) -> Result<Self, NetworkError> {
        let char: Character = character::query::get_character_by_id(state, &reader.char_id).await?;
        let world_id: u8 = session.world_id.ok_or(SessionError::NoWorld(session.id))?;
        let world: World = world::service::get_world_by_id(&world_id)?;
        let channel_id: u8 = session
            .channel_id
            .ok_or(SessionError::NoChannel(session.id))?;
        let channel: Channel = channel::service::get_channel_by_ids(&channel_id, &world_id)?;
        register_pic::service::set_pic(state, &session, &read.pic).await?;
        let ip: Ipv4Addr = settings::get_ip()?;
        Ok(Self {
            char: char.clone(),
            world: world.clone(),
            channel: channel.clone(),
            ip: ip.clone(),
        })
    }
}
