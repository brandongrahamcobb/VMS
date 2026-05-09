use crate::config::settings;
use crate::inc::helpers;
use crate::models::character;
use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::select_char::reader::SelectCharReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct SelectCharStore {
    pub channel: Channel,
    pub char: Character,
    pub octets: [u8; 4],
}

impl SelectCharStore {
    pub async fn store_select_char(
        state: &SharedState,
        session: Session,
        reader: SelectCharReader,
    ) -> Result<Self, NetworkError> {
        let channel: Channel = session.get_channel()?;
        let char: Character =
            character::service::get_character_by_id(state, reader.char_id).await?;
        let addr: String = settings::get_routing_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);
        Ok(Self {
            channel,
            char,
            octets,
        })
    }
}
