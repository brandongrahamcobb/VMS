use crate::models::character::model::Character;
use crate::models::world::model::World;
use crate::models::{channel, character, world};
use crate::net::error::NetworkError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use core::net::Ipv4Addr;

pub struct SelectCharStore {
    pub char: Character,
    pub ip: Ipv4Addr,
    pub world: World,
    pub channel: Channel,
}

impl SelectCharStore {
    pub fn new() -> Self {
        Self
    }

    pub fn store_select_char(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &SelectCharReader,
    ) -> Result<Self, NetworkError> {
        let char: Character = character::query::get_character_by_id(state, &reader.char_id)?;
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
        })
    }
}
