use crate::config::settings;
use crate::inc::helpers;
use crate::models::channel::model::ChannelModel;
use crate::models::character;
use crate::models::character::model::CharacterModel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::register_pic;
use crate::net::packet::handler::register_pic::reader::RegisterPicReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct RegisterPicStore {
    pub channel_model: ChannelModel,
    pub char_model: CharacterModel,
    pub octets: [u8; 4],
}

impl RegisterPicStore {
    pub async fn store_register_pic(
        state: &SharedState,
        session: Session,
        reader: RegisterPicReader,
    ) -> Result<Self, NetworkError> {
        let channel_model: ChannelModel = session.channel.model.clone();
        let char_model: CharacterModel =
            character::query::get_character_model_by_id(state, reader.char_id).await?;
        register_pic::service::set_pic(state, session, reader.pic.clone()).await?;
        let addr: String = settings::get_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);
        Ok(Self {
            char_model,
            channel_model,
            octets,
        })
    }
}
