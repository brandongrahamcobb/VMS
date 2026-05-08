use crate::config::settings;
use crate::inc::helpers;
use crate::models::channel::model::ChannelModel;
use crate::models::character;
use crate::models::character::model::CharacterModel;
use crate::models::world::model::WorldModel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::select_char::reader::SelectCharReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct SelectCharStore {
    pub channel_model: ChannelModel,
    pub char_model: CharacterModel,
    pub octets: [u8; 4],
}

impl SelectCharStore {
    pub async fn store_select_char(
        state: &SharedState,
        session: Session,
        reader: SelectCharReader,
    ) -> Result<Self, NetworkError> {
        let channel_model: ChannelModel = session.channel.model.clone();
        let char_model: CharacterModel =
            character::query::get_character_by_id(state, reader.char_id).await?;
        let world_model: WorldModel = session.world.model.clone();
        let addr: String = settings::get_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr);
        Ok(Self {
            channel_model,
            char_model,
            octets,
        })
    }
}
