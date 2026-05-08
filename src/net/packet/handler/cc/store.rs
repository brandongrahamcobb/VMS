use crate::config::settings;
use crate::inc::helpers;
use crate::models::channel;
use crate::models::channel::model::ChannelModel;
use crate::models::character::model::Character;
use crate::models::world::model::WorldModel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::cc::reader::ChangeChannelReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ChangeChannelStore {
    pub char: Character,
    pub channel: ChannelModel,
    pub octets: [u8; 4],
}

impl ChangeChannelStore {
    pub async fn store_change_channel(
        state: &SharedState,
        session: Session,
        reader: ChangeChannelReader,
    ) -> Result<Self, NetworkError> {
        let world_model: WorldModel = session.world.model.clone();
        let channel = channel::service::get_channel_by_ids(reader.channel_id, world_model.id)?;
        let char = session.char.clone(); // preemptively planning for spawn player equips
        let addr = settings::get_address()?;
        let octets = helpers::convert_to_ip_array(addr);
        Ok(Self {
            char,
            channel,
            octets,
        })
    }
}
