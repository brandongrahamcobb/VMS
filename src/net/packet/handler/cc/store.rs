use crate::config::settings;
use crate::inc::helpers;
use crate::models::character::model::Character;
use crate::models::shroom::channel;
use crate::models::shroom::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::cc::reader::ChangeChannelReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ChangeChannelStore {
    pub char: Character,
    pub channel: Channel,
    pub octets: [u8; 4],
}

impl ChangeChannelStore {
    pub async fn store_change_channel(
        state: &SharedState,
        session: Session,
        reader: ChangeChannelReader,
    ) -> Result<Self, NetworkError> {
        let channel = channel::service::get_channel_by_id(state, reader.channel_id).await?;
        let addr = settings::get_routing_address()?;
        let octets = helpers::convert_to_ip_array(addr);
        let char = session.get_char()?;
        Ok(Self {
            char,
            channel,
            octets,
        })
    }
}
