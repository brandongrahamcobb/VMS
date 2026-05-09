use crate::config::settings;
use crate::inc::helpers;
use crate::models::account::model::Account;
use crate::models::character;
use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::register_pic::reader::RegisterPicReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct RegisterPicStore {
    pub channel: Channel,
    pub char: Character,
    pub octets: [u8; 4],
}

impl RegisterPicStore {
    pub async fn store_register_pic(
        state: &SharedState,
        session: Session,
        reader: RegisterPicReader,
    ) -> Result<Self, NetworkError> {
        let acc: Account = session.get_acc()?;
        let channel: Channel = session.get_channel()?;
        let char: Character =
            character::service::get_character_by_id(state, reader.char_id).await?;
        acc.set_pic(state, reader.pic.clone()).await?;
        let addr: String = settings::get_routing_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);
        Ok(Self {
            char,
            channel,
            octets,
        })
    }
}
